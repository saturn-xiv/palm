#include "phlox/application.hpp"
#include "phlox/docker.hpp"
#include "phlox/services.hpp"

static inline boost::optional<int64_t> get_last_fetched_logs_at(
    std::shared_ptr<soci::session> db, const std::string& id) {
  boost::optional<int64_t> it;
  (*db)
      << R"SQL(SELECT last_fetched_at FROM docker_container_logs WHERE id = :id)SQL",
      soci::use(id, "id"), soci::into(it);
  return it;
}

static inline void set_last_fetched_logs_at(std::shared_ptr<soci::session> db,
                                            const std::string& id,
                                            int64_t last_fetched_at) {
  int c = 0;
  (*db) << R"SQL(SELECT COUNT(*) FROM docker_container_logs WHERE id = :id)SQL",
      soci::use(id, "id"), soci::into(c);
  if (c > 0) {
    (*db)
        << R"SQL(UPDATE docker_container_logs SET last_fetched_at=:last_fetched_at, version=version+1 WHERE id=:id)SQL",
        soci::use(id, "id"), soci::use(last_fetched_at, "last_fetched_at");
  } else {
    (*db)
        << R"SQL(INSERT INTO docker_container_logs(id, last_fetched_at) VALUES(:id, :last_fetched_at))SQL",
        soci::use(id, "id"), soci::use(last_fetched_at, "last_fetched_at");
  }
}

void phlox::Application::docker_logs(const toml::table& config) {
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

  const auto containers = phlox::docker::ps(true);
  for (const auto& container : containers) {
    if (container.State == "created") {
      spdlog::debug("skip created container {}", container.ID);
      continue;
    }
    const auto last_fetched_at = get_last_fetched_logs_at(db, container.ID);
    const int64_t begin =
        last_fetched_at ? last_fetched_at.value() : container.created_at();
    // const int64_t end = container.State == "exited" ? container.ExitedAt :
    // now;
    const int64_t end = now;
    for (int64_t since = begin;; since += interval.count()) {
      const int64_t until = since + interval.count();
      if (until >= end) {
        spdlog::debug("wait for next turn");
        break;
      }

      const auto items = phlox::docker::logs(
          container.ID, static_cast<time_t>(since), static_cast<time_t>(until));
      if (items.empty()) {
        spdlog::debug("empty logs");
      } else {
        spdlog::info("fetch {} logs for {}", items.size(), container.ID);

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
      set_last_fetched_logs_at(db, container.ID, until);
    }
  }

  {
    const auto it =
        search->count<palm::monitoring::v1::PodmanLogsResponse_Item>();
    spdlog::debug("{} total has {} items", index_name, it.value());
  }
}

void phlox::Application::docker_stats(const toml::table& config, bool all) {
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

    const auto items = phlox::docker::stats(all);
    if (items.empty()) {
      return;
    }
    for (const auto& it : items) {
      spdlog::debug("find container {}({})", it.Name, it.ID);

      palm::monitoring::v1::DockerStatisticsResponse_Item x;
      x.set_host(hostname);
      x.set_id(it.ID);
      x.set_name(it.Name);
      x.set_block_io(it.BlockIO);
      x.set_cpu_percent(it.CPUPerc);
      x.set_container(it.Container);
      x.set_mem_percent(it.MemPerc);
      x.set_mem_usage(it.MemUsage);
      x.set_net_io(it.NetIO);
      x.set_pids(it.PIDs);

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

void phlox::Application::docker_ps(const toml::table& config, bool all) {
  if (palm::is_stopped()) {
    return;
  }

  const std::string hostname = boost::asio::ip::host_name();
  auto search = this->opensearch(config);

  const auto index_name =
      search->index_name<palm::monitoring::v1::DockerContainersResponse_Item>();

  palm::opensearch::requests::bulk_index::Action bulk{
      .index = {._index = index_name}};

  std::stringstream body;
  {
    const auto now = std::chrono::system_clock::now();
    const time_t seconds = std::chrono::system_clock::to_time_t(now);

    const auto items = phlox::docker::ps(all);
    if (items.empty()) {
      return;
    }
    for (const auto& it : items) {
      spdlog::debug("find container {}({})", it.ID, it.Names);

      palm::monitoring::v1::DockerContainersResponse_Item x;
      x.set_host(hostname);
      {
        auto y = x.mutable_created_at();
        y->set_seconds(it.created_at());
        y->set_nanos(0);
      }
      x.set_id(it.ID);
      x.set_image(it.Image);
      x.set_labels(it.Labels);
      x.set_local_volumes(it.LocalVolumes);
      x.set_mounts(it.Mounts);
      x.set_names(it.Names);
      x.set_networks(it.Networks);
      if (it.Platform) {
        x.set_platform(it.Platform.value());
      }
      x.set_ports(it.Ports);
      x.set_running_for(it.RunningFor);
      x.set_size(it.Size);
      x.set_state(it.State);
      x.set_status(it.Status);

      {
        auto y = x.mutable_updated_at();
        y->set_seconds(seconds);
        y->set_nanos(0);
      }

      bulk.index._id = std::format("{}.{}", hostname, it.ID);
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
