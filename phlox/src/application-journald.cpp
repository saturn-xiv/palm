#include "palm/utils.hpp"
#include "phlox/application.hpp"
#include "phlox/services.hpp"
#include "phlox/systemd.hpp"

static inline boost::optional<int64_t> get_last_fetched_logs_at(
    std::shared_ptr<soci::session> db, const std::string& name) {
  boost::optional<int64_t> it;
  (*db)
      << R"SQL(SELECT last_fetched_at FROM systemd_service_logs WHERE name = :name)SQL",
      soci::use(name, "name"), soci::into(it);
  return it;
}

static inline void set_last_fetched_logs_at(std::shared_ptr<soci::session> db,
                                            const std::string& name,
                                            int64_t last_fetched_at) {
  int c = 0;
  (*db)
      << R"SQL(SELECT COUNT(*) FROM systemd_service_logs WHERE name = :name)SQL",
      soci::use(name, "name"), soci::into(c);
  if (c > 0) {
    (*db)
        << R"SQL(UPDATE systemd_service_logs SET last_fetched_at=:last_fetched_at, version=version+1 WHERE name=:name)SQL",
        soci::use(name, "name"), soci::use(last_fetched_at, "last_fetched_at");
  } else {
    (*db)
        << R"SQL(INSERT INTO systemd_service_logs(name, last_fetched_at) VALUES(:name, :last_fetched_at))SQL",
        soci::use(name, "name"), soci::use(last_fetched_at, "last_fetched_at");
  }
}

void phlox::Application::systemd_journal(const toml::table& config,
                                         const std::string& service_name,
                                         bool user_scope) {
  if (palm::is_stopped()) {
    return;
  }
  if (!user_scope && !palm::is_root()) {
    spdlog::error("must have root privileges");
    return;
  }

  const std::chrono::seconds interval =
      std::chrono::duration_cast<std::chrono::seconds>(std::chrono::minutes{5});

  const auto now = palm::epoch_in_seconds();
  auto db = this->db(config);
  auto search = this->opensearch(config);
  const auto index_name =
      search->index_name<palm::monitoring::v1::SystemdJournalResponse_Item>();

  palm::opensearch::requests::bulk_index::Action bulk{
      .index = {._index = index_name}};

  // TODO check systemd service exists
  const int64_t booted_at = palm::booted_at().value();
  {
    const auto last_fetched_at = get_last_fetched_logs_at(db, service_name);
    const int64_t begin = last_fetched_at ? last_fetched_at.value() : booted_at;
    const int64_t end = now;
    for (int64_t since = begin;; since += interval.count()) {
      const int64_t until = since + interval.count();
      if (until >= end) {
        spdlog::debug("wait for next turn");
        break;
      }

      const auto items = phlox::systemd::logs(service_name, user_scope,
                                              static_cast<time_t>(since),
                                              static_cast<time_t>(until));
      if (items.empty()) {
        spdlog::debug("empty logs");

      } else {
        spdlog::info("fetch {} logs for {}", items.size(), service_name);

        std::stringstream body;
        for (const auto& it : items) {
          if (!it.MESSAGE.content) {
            continue;
          }

          palm::monitoring::v1::SystemdJournalResponse_Item x;
          x.set_host(it._HOSTNAME);
          x.set_name(it.UNIT);
          x.set_message(it.MESSAGE.content.value());
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
      set_last_fetched_logs_at(db, service_name, until);
    }
  }
  {
    const auto it =
        search->count<palm::monitoring::v1::SystemdJournalResponse_Item>();
    spdlog::debug("{} total has {} items", index_name, it.value());
  }
}
