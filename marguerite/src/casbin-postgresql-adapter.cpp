#include "marguerite/casbin.hpp"

#include <boost/algorithm/string/join.hpp>

#include <casbin/exception/casbin_adapter_exception.h>
#include <casbin/exception/io_exception.h>
#include <casbin/exception/unsupported_operation_exception.h>
#include <casbin/util/util.h>

marguerite::casbin::PostgreSqlAdapter::PostgreSqlAdapter(
    const std::string& host, uint16_t port, const std::string& db_name,
    const std::string& user, const std::optional<std::string> password)
    : _is_filtered(false) {
  std::stringstream ss;
  ss << "host=" << host << " port=" << port << " dbname=" << db_name
     << " user=" << user;
  if (password) {
    ss << " password=" << password.value();
  }
  ss << " sslmode=disable";
  this->_connection = pqxx::connection(ss.str());
}

void marguerite::casbin::PostgreSqlAdapter::LoadPolicy(
    const std::shared_ptr<::casbin::Model>& model) {
  const std::lock_guard<std::mutex> lock(this->_locker);

  {
    pqxx::work tx{this->_connection};
    tx.exec0(R"SQL(
CREATE TABLE IF NOT EXISTS casbin_rule(
    p_type VARCHAR(32)  DEFAULT '' NOT NULL,
    v0     VARCHAR(255) DEFAULT '' NOT NULL,
    v1     VARCHAR(255) DEFAULT '' NOT NULL,
    v2     VARCHAR(255) DEFAULT '' NOT NULL,
    v3     VARCHAR(255) DEFAULT '' NOT NULL,
    v4     VARCHAR(255) DEFAULT '' NOT NULL,
    v5     VARCHAR(255) DEFAULT '' NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_casbin_rule ON casbin_rule(p_type, v0, v1);
)SQL");
    tx.commit();
  }
  {
    pqxx::work tx{this->_connection};
    for (
        auto [p_type, v0, v1, v2, v3, v4, v5] :
        tx.query<std::string, std::string, std::string, std::string,
                 std::string, std::string, std::string>(
            R"SQL(SELECT p_type, v0, v1, v2, v3, v4, v5 FROM casbin_rule)SQL")) {
      const std::string sec = p_type.substr(0, 1);

      if (model->m.find(sec) == model->m.end()) {
        model->m[sec] = ::casbin::AssertionMap();
      }

      std::vector<std::string> line = {v0, v1};
      if (v2 != "") {
        line.push_back(v2);
        if (v3 != "") {
          line.push_back(v3);
          if (v4 != "") {
            line.push_back(v4);
            if (v5 != "") {
              line.push_back(v5);
            }
          }
        }
      }
      model->m[sec].assertion_map[p_type]->policy.emplace(line);
    }
  }
}
void marguerite::casbin::PostgreSqlAdapter::SavePolicy(
    const std::shared_ptr<::casbin::Model>& model) {
  const std::lock_guard<std::mutex> lock(this->_locker);
  pqxx::work tx{this->_connection};
  this->save(tx, model, "p");
  this->save(tx, model, "g");
  tx.commit();
}
void marguerite::casbin::PostgreSqlAdapter::AddPolicy(
    std::string sec, std::string p_type, std::vector<std::string> rule) {
  const std::lock_guard<std::mutex> lock(this->_locker);
  pqxx::work tx{this->_connection};
  switch (rule.size()) {
    case 2:
      this->insert(tx, p_type, rule[0], rule[1]);
      break;
    case 3:
      this->insert(tx, p_type, rule[0], rule[1], rule[2]);
      break;
    case 4:
      this->insert(tx, p_type, rule[0], rule[1], rule[2], rule[3]);
      break;
    case 5:
      this->insert(tx, p_type, rule[0], rule[1], rule[2], rule[3], rule[4]);
    case 6:
      this->insert(tx, p_type, rule[0], rule[1], rule[2], rule[3], rule[4],
                   rule[6]);
      break;
    default:
      spdlog::error("unknown policy({}) ({},{})", sec, p_type,
                    boost::algorithm::join(rule, ","));
      break;
  }
  tx.commit();
}
void marguerite::casbin::PostgreSqlAdapter::RemovePolicy(
    std::string sec, std::string p_type, std::vector<std::string> rule) {
  const std::lock_guard<std::mutex> lock(this->_locker);
  pqxx::work tx{this->_connection};
  switch (rule.size()) {
    case 2:
      this->remove(tx, p_type, rule[0], rule[1]);
      break;
    case 3:
      this->remove(tx, p_type, rule[0], rule[1], rule[2]);
      break;
    case 4:
      this->remove(tx, p_type, rule[0], rule[1], rule[2], rule[3]);
      break;
    case 5:
      this->remove(tx, p_type, rule[0], rule[1], rule[2], rule[3], rule[4]);
    case 6:
      this->remove(tx, p_type, rule[0], rule[1], rule[2], rule[3], rule[4],
                   rule[6]);
      break;
    default:
      spdlog::error("unknown policy({}) ({},{})", sec, p_type,
                    boost::algorithm::join(rule, ","));
      break;
  }
  tx.commit();
}
void marguerite::casbin::PostgreSqlAdapter::RemoveFilteredPolicy(
    std::string sec, std::string p_type, int field_index,
    std::vector<std::string> field_values) {
  const std::lock_guard<std::mutex> lock(this->_locker);
  pqxx::work tx{this->_connection};
  throw ::casbin::UnsupportedOperationException("not implemented");
}
bool marguerite::casbin::PostgreSqlAdapter::IsFiltered() {
  return this->_is_filtered;
}
bool marguerite::casbin::PostgreSqlAdapter::IsValid() {
  const std::lock_guard<std::mutex> lock(this->_locker);
  pqxx::work tx{this->_connection};
  pqxx::row row = tx.exec1(R"SQL(SELECT 1)SQL");
  tx.commit();
  return row[0].as<int>() == 1;
}

void marguerite::casbin::PostgreSqlAdapter::insert(
    pqxx::work& tx, const std::string& p_type, const std::string& v0,
    const std::string& v1, const std::string& v2, const std::string& v3,
    const std::string& v4, const std::string& v5) {
  spdlog::info("create policy ({},{},{},{},{},{},{})", p_type, v0, v1, v2, v3,
               v4, v5);
  tx.exec_params(
      R"SQL(INSERT INTO casbin_rule(p_type, v0, v1, v2, v3, v4, v5) VALUES($1, $2, $3, $4, $5, $6, $7))SQL",
      p_type, v0, v1, v2, v3, v4, v5);
}

bool marguerite::casbin::PostgreSqlAdapter::is_exist(
    pqxx::work& tx, const std::string& p_type, const std::string& v0,
    const std::string& v1, const std::string& v2, const std::string& v3,
    const std::string& v4, const std::string& v5) {
  pqxx::result rst = tx.exec_params(
      R"SQL(SELECT COUNT(*) FROM casbin_rule WHERE p_type=$1 AND v0=$1 AND
        v1=$3 AND v2=$4 AND v3=$5 AND v4=$6 AND v5=$7)SQL",
      p_type, v0, v1, v2, v3, v4, v5);
  return rst[0][0].as<int>() > 0;
}
void marguerite::casbin::PostgreSqlAdapter::remove(
    pqxx::work& tx, const std::string& p_type, const std::string& v0,
    const std::string& v1, const std::string& v2, const std::string& v3,
    const std::string& v4, const std::string& v5) {
  spdlog::info("remove policy ({},{},{},{},{},{},{})", p_type, v0, v1, v2, v3,
               v4, v5);
  tx.exec_params(
      R"SQL(DELETE FROM casbin_rule WHERE p_type=$1 AND v0=$1 AND v1=$3 AND v2=$4 AND v3=$5 AND v4=$6 AND v5=$7)SQL",
      p_type, v0, v1, v2, v3, v4, v5);
}
void marguerite::casbin::PostgreSqlAdapter::save(
    pqxx::work& tx, const std::shared_ptr<::casbin::Model>& model,
    const std::string& sec) {
  for (std::unordered_map<std::string,
                          std::shared_ptr<::casbin::Assertion>>::iterator it =
           model->m[sec].assertion_map.begin();
       it != model->m[sec].assertion_map.end(); it++) {
    for (auto& rule : it->second->policy) {
      switch (rule.size()) {
        case 2:
          if (!this->is_exist(tx, it->first, rule[0], rule[1])) {
            this->insert(tx, it->first, rule[0], rule[1]);
          }
          break;
        case 3:
          if (!this->is_exist(tx, it->first, rule[0], rule[1], rule[2])) {
            this->insert(tx, it->first, rule[0], rule[1], rule[2]);
          }
          break;
        case 4:
          if (!this->is_exist(tx, it->first, rule[0], rule[1], rule[2],
                              rule[3])) {
            this->insert(tx, it->first, rule[0], rule[1], rule[2], rule[3]);
          }
          break;
        case 5:
          if (!this->is_exist(tx, it->first, rule[0], rule[1], rule[2], rule[3],
                              rule[4])) {
            this->insert(tx, it->first, rule[0], rule[1], rule[2], rule[3],
                         rule[4]);
          }
        case 6:
          if (!this->is_exist(tx, it->first, rule[0], rule[1], rule[2], rule[3],
                              rule[4], rule[6])) {
            this->insert(tx, it->first, rule[0], rule[1], rule[2], rule[3],
                         rule[4], rule[6]);
          }
          break;
        default:
          spdlog::error("unknown policy({}) ({},{})", sec, it->first,
                        boost::algorithm::join(rule, ","));
          break;
      }
    }
  }
}
