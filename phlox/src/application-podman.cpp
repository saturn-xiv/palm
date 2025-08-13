#include "phlox/application.hpp"
#include "phlox/podman.hpp"
#include "phlox/services.hpp"

static inline boost::optional<int64_t> get_last_fetched_logs_at(
    std::shared_ptr<soci::session> db, const std::string& id) {
  boost::optional<int64_t> it;
  (*db)
      << R"SQL(SELECT last_fetched_at FROM podman_container_logs WHERE id = :id)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}

static inline void set_last_fetched_logs_at(std::shared_ptr<soci::session> db,
                                            const std::string& id,
                                            int64_t last_fetched_at) {
  int c = 0;
  (*db) << R"SQL(SELECT COUNT(*) FROM podman_container_logs WHERE id = :id)SQL",
      soci::use(id, "id"), soci::into(c);
  if (c > 0) {
    (*db)
        << R"SQL(UPDATE podman_container_logs SET last_fetched_at=:last_fetched_at, version=version+1 WHERE id=:id)SQL",
        soci::use(id, "id"), soci::use(last_fetched_at, "last_fetched_at");
  } else {
    (*db)
        << R"SQL(INSERT INTO podman_container_logs(id, last_fetched_at) VALUES(:id, :last_fetched_at))SQL",
        soci::use(id, "id"), soci::use(last_fetched_at, "last_fetched_at");
  }
}

void phlox::Application::podman_logs(const toml::table& config) {
  if (palm::is_stopped()) {
    return;
  }
  const std::chrono::seconds interval =
      std::chrono::duration_cast<std::chrono::seconds>(std::chrono::minutes{5});

  const auto now = palm::epoch_in_seconds();
  auto db = this->db(config);
  auto search = this->opensearch(config);

  const auto index_name =
      search->index_name<palm::monitoring::v1::PodmanLogsResponse_Item>();

  palm::opensearch::requests::bulk_index::Action bulk{
      .index = {._index = index_name}};

  const auto containers = phlox::podman::ps(true);
  for (const auto& container : containers) {
    if (container.State == "created") {
      spdlog::debug("skip created container {}", container.Id);
      continue;
    }
    const auto last_fetched_at = get_last_fetched_logs_at(db, container.Id);
    const int64_t begin =
        last_fetched_at ? last_fetched_at.value() : container.Created;
    const int64_t end = container.Exited ? container.ExitedAt : now;
    for (int64_t since = begin;; since += interval.count()) {
      const int64_t until = since + interval.count();
      if (until >= end) {
        spdlog::debug("wait for next turn");
        break;
      }

      const auto items = phlox::podman::logs(
          container.Id, static_cast<time_t>(since), static_cast<time_t>(until));
      if (items.empty()) {
        spdlog::debug("empty logs");
      } else {
        spdlog::info("fetch {} logs for {}", items.size(), container.Id);

        std::stringstream body;
        for (const auto& it : items) {
          if (!it.MESSAGE.has_value() || !it.MESSAGE->content.has_value()) {
            continue;
          }

          palm::monitoring::v1::PodmanLogsResponse_Item x;
          x.set_host(it._HOSTNAME);
          x.set_id(it.CONTAINER_ID);
          x.set_full_id(it.CONTAINER_ID_FULL);
          x.set_name(it.CONTAINER_NAME);
          x.set_message(it.MESSAGE->content.value());
          {
            int64_t ts = std::stol(it.__REALTIME_TIMESTAMP);
            auto y = x.mutable_created_at();
            y->set_seconds(ts / 1000000);
            y->set_nanos((ts % 1000000) * 1000);
          }

          bulk.index._id = std::format("{}.{}.{}", it._MACHINE_ID,
                                       it.__SEQNUM_ID, it.__SEQNUM);
          nlohmann::json act = bulk;
          body << act.dump() << "\n";

          const auto buf = palm::to_json(x);
          body << buf.value() << "\n";
        }

        const auto req = body.str();
        if (req.empty()) {
          spdlog::warn("skip for empty bulk body");
        } else {
          spdlog::debug("{}", req);

          const auto res = search->post("_bulk", req);
          {
            const auto body = res.value();
            auto js = nlohmann::json::parse(body);
            auto it =
                js.template get<palm::opensearch::responses::bulk::Item>();
            if (it.errors) {
              spdlog::error("{}", body);
              return;
            }
          }
        }
      }
      set_last_fetched_logs_at(db, container.Id, until);
    }
  }

  {
    const auto it =
        search->count<palm::monitoring::v1::PodmanLogsResponse_Item>();
    spdlog::debug("{} total has {} items", index_name, it.value());
  }
}

void phlox::Application::podman_stats(const toml::table& config, bool all) {
  if (palm::is_stopped()) {
    return;
  }
  const std::string hostname = boost::asio::ip::host_name();
  auto search = this->opensearch(config);

  const auto index_name =
      search->index_name<palm::monitoring::v1::PodmanStatisticsResponse_Item>();

  palm::opensearch::requests::bulk_create::Action bulk{
      .create = {._index = index_name}};
  std::stringstream body;

  {
    const auto now = std::chrono::system_clock::now();
    const time_t seconds = std::chrono::system_clock::to_time_t(now);

    const auto items = phlox::podman::stats(all);
    if (items.empty()) {
      return;
    }
    for (const auto& it : items) {
      spdlog::debug("find container {}({})", it.name, it.id);

      palm::monitoring::v1::PodmanStatisticsResponse_Item x;
      x.set_host(hostname);
      x.set_id(it.id);
      x.set_name(it.name);
      x.set_cpu_time(it.cpu_time);
      x.set_cpu_percent(it.cpu_percent);
      x.set_avg_cpu(it.avg_cpu);
      x.set_mem_usage(it.mem_usage);
      x.set_mem_percent(it.mem_percent);
      x.set_net_io(it.net_io);
      x.set_block_io(it.block_io);
      x.set_pids(it.pids);

      {
        auto y = x.mutable_created_at();
        y->set_seconds(seconds);
        y->set_nanos(0);
      }

      nlohmann::json act = bulk;
      body << act.dump() << "\n";

      const auto buf = palm::to_json(x);
      body << buf.value() << "\n";
    }
  }
  spdlog::debug("{}", body.str());
  const auto res = search->post("_bulk", body.str());
  {
    const auto body = res.value();
    auto js = nlohmann::json::parse(body);
    auto it = js.template get<palm::opensearch::responses::bulk::Item>();
    if (it.errors) {
      spdlog::error("{}", body);
      return;
    }
  }
  {
    const auto it =
        search->count<palm::monitoring::v1::PodmanStatisticsResponse_Item>();
    spdlog::debug("{} total has {} items", index_name, it.value());
  }
}
void phlox::Application::podman_ps(const toml::table& config, bool all) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string hostname = boost::asio::ip::host_name();
  auto search = this->opensearch(config);

  const auto index_name =
      search->index_name<palm::monitoring::v1::PodmanContainersResponse_Item>();

  palm::opensearch::requests::bulk_index::Action bulk{
      .index = {._index = index_name}};

  std::stringstream body;
  {
    const auto now = std::chrono::system_clock::now();
    const time_t seconds = std::chrono::system_clock::to_time_t(now);

    const auto items = phlox::podman::ps(all);
    if (items.empty()) {
      return;
    }
    for (const auto& it : items) {
      spdlog::debug("find container {}({})", it.Id,
                    boost::algorithm::join(it.Names, ","));

      palm::monitoring::v1::PodmanContainersResponse_Item x;
      x.set_host(hostname);
      x.set_id(it.Id);
      x.set_image(it.Image);
      x.set_image_id(it.ImageID);
      x.set_pid(it.Pid);
      x.set_state(it.State);
      x.set_status(it.Status);
      x.set_started_at(it.StartedAt);
      x.set_exited(it.Exited);
      x.set_exited_at(it.ExitedAt);
      x.set_exit_code(it.ExitCode);
      x.set_created_at(it.CreatedAt);
      x.set_created(it.Created);
      x.mutable_names()->Add(it.Names.begin(), it.Names.end());
      x.mutable_mounts()->Add(it.Mounts.begin(), it.Mounts.end());
      x.mutable_command()->Add(it.Command.begin(), it.Command.end());

      x.mutable_labels()->insert(it.Labels.begin(), it.Labels.end());
      /*
      {
        auto labels = x->mutable_labels();
        for (const auto& [k, v] : it.Labels) {
          spdlog::debug("{} => {} {} {}", k, v, labels == nullptr,
                        labels->size());
          // (*labels)[k] = v;
          labels->insert({k, v});
        }
      }
      */

      {
        auto y = x.mutable_updated_at();
        y->set_seconds(seconds);
        y->set_nanos(0);
      }

      bulk.index._id = std::format("{}.{}", hostname, it.Id);
      nlohmann::json act = bulk;
      body << act.dump() << "\n";

      const auto buf = palm::to_json(x);
      body << buf.value() << "\n";
    }
  }
  spdlog::debug("{}", body.str());
  const auto res = search->post("_bulk", body.str());
  {
    const auto body = res.value();
    auto js = nlohmann::json::parse(body);
    auto it = js.template get<palm::opensearch::responses::bulk::Item>();
    if (it.errors) {
      spdlog::error("{}", body);
      return;
    }
  }
  {
    const auto it =
        search->count<palm::monitoring::v1::PodmanContainersResponse_Item>();
    spdlog::debug("{} total has {} items", index_name, it.value());
  }
}
