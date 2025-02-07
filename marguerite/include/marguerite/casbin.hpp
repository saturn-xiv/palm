#pragma once

#include <casbin/persist/adapter.h>
#include <spdlog/spdlog.h>
#include <pqxx/pqxx>
#include <toml++/toml.hpp>

namespace marguerite {

namespace casbin {

class PostgreSqlAdapter : virtual public ::casbin::Adapter {
 public:
  PostgreSqlAdapter(const toml::table& config)
      : PostgreSqlAdapter(config["host"].value<std::string>().value(),
                          config["port]"].value<uint16_t>().value(),
                          config["db-name"].value<std::string>().value(),
                          config["user"].value<std::string>().value(),
                          config["password"].value<std::string>()) {}
  // https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
  PostgreSqlAdapter(const std::string& host, uint16_t port,
                    const std::string& db_name,
                    const std::string& user = "postgres",
                    const std::optional<std::string> password = std::nullopt);

  void LoadPolicy(const std::shared_ptr<::casbin::Model>& model) override;
  void SavePolicy(const std::shared_ptr<::casbin::Model>& model) override;
  void AddPolicy(std::string sec, std::string p_type,
                 std::vector<std::string> rule) override;
  void RemovePolicy(std::string sec, std::string p_type,
                    std::vector<std::string> rule) override;
  void RemoveFilteredPolicy(std::string sec, std::string p_type,
                            int field_index,
                            std::vector<std::string> field_values) override;
  bool IsFiltered() override;
  bool IsValid() override;

 private:
  void save(pqxx::work& tx, const std::shared_ptr<::casbin::Model>& model,
            const std::string& sec);
  bool is_exist(pqxx::work& tx, const std::string& p_type,
                const std::string& v0, const std::string& v1,
                const std::string& v2 = "", const std::string& v3 = "",
                const std::string& v4 = "", const std::string& v5 = "");
  void insert(pqxx::work& tx, const std::string& p_type, const std::string& v0,
              const std::string& v1, const std::string& v2 = "",
              const std::string& v3 = "", const std::string& v4 = "",
              const std::string& v5 = "");
  void remove(pqxx::work& tx, const std::string& p_type, const std::string& v0,
              const std::string& v1, const std::string& v2 = "",
              const std::string& v3 = "", const std::string& v4 = "",
              const std::string& v5 = "");
  pqxx::connection _connection;
  bool _is_filtered;
  std::mutex _locker;
};
}  // namespace casbin
}  // namespace marguerite
