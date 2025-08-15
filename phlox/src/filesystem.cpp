#include "phlox/filesystem.hpp"
#include "phlox/services.hpp"

#include <boost/asio/ip/host_name.hpp>
#include <boost/exception/diagnostic_information.hpp>

#include <sys/inotify.h>
#include <sys/types.h>
#include <cerrno>

phlox::monitoring::logging::Source::Source()
    : _hostname(boost::asio::ip::host_name()) {}

// https://developer.ibm.com/tutorials/l-ubuntu-inotify/
phlox::monitoring::logging::FilesystemNotify::FilesystemNotify() : Source() {
  this->_notify_id = inotify_init();
  if (this->_notify_id < 0) {
    spdlog::error("init inotify({}): {}", errno, strerror(errno));
    throw std::runtime_error("init inotify");
  }
}
phlox::monitoring::logging::FilesystemNotify::~FilesystemNotify() {
  for (auto [wd, file] : this->_targets) {
    spdlog::info("remove watch of {}", file.string());
    inotify_rm_watch(this->_notify_id, wd);
  }
  spdlog::info("close notify");
  close(this->_notify_id);
}
void phlox::monitoring::logging::FilesystemNotify::register_(
    const std::filesystem::path& file) {
  std::lock_guard<std::mutex> lock(this->_mutex);

  auto wd = inotify_add_watch(this->_notify_id, file.c_str(),
                              IN_MODIFY | IN_CREATE | IN_DELETE);
  if (wd < 0) {
    spdlog::error("file watch({}, {}): {}", errno, file.string(),
                  strerror(errno));
    return;
  }

  if (std::filesystem::is_regular_file(file)) {
    this->load(file);
  } else if (std::filesystem::is_directory(file)) {
    for (const auto& entry :
         std::filesystem::recursive_directory_iterator(file)) {
      if (entry.is_regular_file()) {
        auto file = entry.path();
        this->load(file);
      }
    }
  } else {
    spdlog::error("unknown file type: {}", file.string());
    return;
  }
  this->_targets[wd] = file;
}

static inline void save_logs(
    std::shared_ptr<palm::opensearch::Client> search,
    std::vector<palm::monitoring::v1::FileSystemLogsResponse_Item>& items) {
  if (items.empty()) {
    spdlog::warn("empty logs list");
    return;
  }
  const auto index_name =
      search->index_name<palm::monitoring::v1::FileSystemLogsResponse_Item>();

  palm::opensearch::requests::bulk_index::Action bulk{
      .index = {._index = index_name}};
  std::stringstream body;
  for (const auto& it : items) {
    nlohmann::json act = bulk;
    body << act.dump() << "\n";
    const auto buf = palm::to_json(it);
    body << buf.value() << "\n";
  }

  const auto req = body.str();
  spdlog::debug("{}", req);

  const auto res = search->post("_bulk", req);
  {
    const auto body = res.value();
    auto js = nlohmann::json::parse(body);
    auto it = js.template get<palm::opensearch::responses::bulk::Item>();
    if (it.errors) {
      spdlog::error("{}", body);
      return;
    }
  }
}

void phlox::monitoring::logging::FilesystemNotify::execute(
    std::shared_ptr<palm::opensearch::Client> search) {
  const auto event_size = sizeof(struct inotify_event);
  const auto buf_len = 1024 * (event_size + 16);
  char buffer[buf_len];

  std::lock_guard<std::mutex> lock(this->_mutex);
  auto length = read(this->_notify_id, buffer, buf_len);
  std::vector<palm::monitoring::v1::FileSystemLogsResponse_Item> items;

  if (length < 0) {
    spdlog::error("read notify buffer({}): {}", errno, strerror(errno));
    return;
  }
  auto i = 0;
  while (i < length) {
    struct inotify_event* event = (struct inotify_event*)&buffer[i];
    const auto file = this->_targets.at(event->wd);
    if (event->len == 0) {
      if (event->mask & IN_CREATE) {
        spdlog::debug("created file {}", file.string());
        this->load(file);
      } else if (event->mask & IN_MODIFY) {
        spdlog::debug("modified file {}", file.string());
        auto buf = this->load(file);
        items.insert(items.end(), buf.begin(), buf.end());
      }

    } else if (event->len > 0 && !(event->mask & IN_ISDIR)) {
      auto it = file / event->name;
      if (event->mask & IN_CREATE) {
        spdlog::debug("created file {}", file.string());
        this->load(it);
      } else if (event->mask & IN_MODIFY) {
        spdlog::debug("modified file {}", it.string());
        auto buf = this->load(it);
        items.insert(items.end(), buf.begin(), buf.end());
      }
    }

    i += event_size + event->len;
  }

  save_logs(search, items);
}

void phlox::monitoring::logging::StdinSource::execute(
    std::shared_ptr<palm::opensearch::Client> search) {
  std::vector<palm::monitoring::v1::FileSystemLogsResponse_Item> items;

  std::string line;
  while (std::getline(std::cin, line)) {
    palm::monitoring::v1::FileSystemLogsResponse_Item it;
    it.set_host(this->_hostname);
    it.set_file("stdin");
    it.set_line(line);
    {
      auto at = it.mutable_created_at();
      palm::now(at);
    }
    items.push_back(it);

    if (items.size() > 128) {
      save_logs(search, items);
      items.clear();
    }
  }
}

void phlox::monitoring::LoggingScratcher::launch(
    std::shared_ptr<palm::opensearch::Client> search,
    std::chrono::seconds ttl) {
  std::vector<std::shared_ptr<std::thread>> pool;
  {
    std::lock_guard<std::mutex> lock(this->_mutex);
    for (auto& it : this->_nodes) {
      std::shared_ptr<std::thread> t = std::make_shared<std::thread>([&] {
        while (true) {
          try {
            it->execute(search);
          } catch (...) {
            spdlog::error("{}",
                          boost::current_exception_diagnostic_information());
          }
          std::this_thread::sleep_for(ttl);
        }
      });
      pool.push_back(t);
    }
  }
  if (!pool.empty()) {
    spdlog::info("start a logging scratcher progress");
    for (auto& it : pool) {
      it->join();
    }
  }
}
