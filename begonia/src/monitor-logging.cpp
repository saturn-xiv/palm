#include "basil/monitor.hpp"

#include <boost/asio/ip/host_name.hpp>
#include <boost/exception/diagnostic_information.hpp>

#include <sys/inotify.h>
#include <sys/types.h>
#include <cerrno>

basil::monitor::logging::Source::Source()
    : _hostname(boost::asio::ip::host_name()) {}

// https://developer.ibm.com/tutorials/l-ubuntu-inotify/
basil::monitor::logging::FilesystemNotify::FilesystemNotify() : Source() {
  this->_notify_id = inotify_init();
  if (this->_notify_id < 0) {
    BOOST_LOG_TRIVIAL(error)
        << "init inotify(" << errno << ") " << ": " << strerror(errno);
    throw std::runtime_error("init inotify");
  }
}
basil::monitor::logging::FilesystemNotify::~FilesystemNotify() {
  for (auto [wd, file] : this->_targets) {
    BOOST_LOG_TRIVIAL(info) << "remove watch of " << file.string();
    inotify_rm_watch(this->_notify_id, wd);
  }
  BOOST_LOG_TRIVIAL(info) << "close notify";
  close(this->_notify_id);
}
void basil::monitor::logging::FilesystemNotify::register_(
    const std::filesystem::path& file) {
  std::lock_guard<std::mutex> lock(this->_mutex);

  auto wd = inotify_add_watch(this->_notify_id, file.c_str(),
                              IN_MODIFY | IN_CREATE | IN_DELETE);
  if (wd < 0) {
    BOOST_LOG_TRIVIAL(error)
        << "file watch(" << errno << ") " << file << ": " << strerror(errno);
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
    BOOST_LOG_TRIVIAL(error) << "unknown file type: " << file.string();
    return;
  }
  this->_targets[wd] = file;
}

void basil::monitor::logging::FilesystemNotify::execute(
    std::shared_ptr<basil::opensearch::Client> search) {
  const auto event_size = sizeof(struct inotify_event);
  const auto buf_len = 1024 * (event_size + 16);
  char buffer[buf_len];

  std::lock_guard<std::mutex> lock(this->_mutex);
  auto length = read(this->_notify_id, buffer, buf_len);
  std::vector<std::tuple<std::filesystem::path, std::string, uint64_t>> items;

  if (length < 0) {
    BOOST_LOG_TRIVIAL(error)
        << "read notify buffer(" << errno << "): " << strerror(errno);
    return;
  }
  auto i = 0;
  while (i < length) {
    struct inotify_event* event = (struct inotify_event*)&buffer[i];
    const auto file = this->_targets.at(event->wd);
    if (event->len == 0) {
      if (event->mask & IN_CREATE) {
        BOOST_LOG_TRIVIAL(debug) << "created file " << file.string();
        this->load(file);
      } else if (event->mask & IN_MODIFY) {
        BOOST_LOG_TRIVIAL(debug) << "modified file " << file.string();
        auto buf = this->load(file);
        items.insert(items.end(), buf.begin(), buf.end());
      }

    } else if (event->len > 0 && !(event->mask & IN_ISDIR)) {
      auto it = file / event->name;
      if (event->mask & IN_CREATE) {
        BOOST_LOG_TRIVIAL(debug) << "created file " << file.string();
        this->load(it);
      } else if (event->mask & IN_MODIFY) {
        BOOST_LOG_TRIVIAL(debug) << "modified file " << it.string();
        auto buf = this->load(it);
        items.insert(items.end(), buf.begin(), buf.end());
      }
    }

    i += event_size + event->len;
  }

  {
    basil::monitor::logging::Item log = {.host = this->_hostname};
    for (const auto [f, m, c] : items) {
      log.file = f.string();
      log.message = m;
      boost::algorithm::trim(log.message);
      log.created_at = c;
      if (!log.message.empty()) {
        search->index_document(log);
      }
    }
  }
}

void basil::monitor::logging::StdinSource::execute(
    std::shared_ptr<basil::opensearch::Client> search) {
  basil::monitor::logging::Item it = {.host = this->_hostname, .file = "stdin"};
  std::string line;
  while (std::getline(std::cin, it.message)) {
    boost::algorithm::trim(it.message);
    it.created_at = basil::monitor::logging::Item::now();
    if (!it.message.empty()) {
      search->index_document(it);
    }
  }
}

void basil::monitor::LoggingScratcher::launch(
    std::shared_ptr<basil::opensearch::Client> search,
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
            BOOST_LOG_TRIVIAL(error)
                << boost::current_exception_diagnostic_information();
          }
          std::this_thread::sleep_for(ttl);
        }
      });
      pool.push_back(t);
    }
  }
  BOOST_LOG_TRIVIAL(info) << "start a logging scratcher progress";
  for (auto& it : pool) {
    it->join();
  }
}
