#include "palm/rbac.hpp"

#include <boost/algorithm/string/join.hpp>

static void save_policy(soci::session& db, const std::string& p_type,
                        const std::vector<std::string>& rule) {
  if (rule.size() != 6) {
    spdlog::warn("ignore policy rule: {}", boost::algorithm::join(rule, ","));
    return;
  }
  db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3, v4, v5) VALUES(:ptype, :v0, :v1, :v2, :v3, :v4, :v5))SQL",
      soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
      soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
      soci::use(rule[3], "v3"), soci::use(rule[4], "v4"),
      soci::use(rule[5], "v5");
}
static void clear_policies(soci::session& db) {
  db << R"SQL(DELETE FROM casbin_rule)SQL";
}
static void remove_policy(soci::session& db, const std::string& p_type,
                          const std::vector<std::string>& rule) {
  if (rule.size() != 6) {
    spdlog::warn("ignore policy rule: {}", boost::algorithm::join(rule, ","));
    return;
  }
  db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4=:v4 AND v5=:v5))SQL",
      soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
      soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
      soci::use(rule[3], "v3"), soci::use(rule[4], "v4"),
      soci::use(rule[5], "v5");
}

void palm::casbin::PostgreSQLAdapter::LoadPolicy(
    const std::shared_ptr<::casbin::Model>& model) {
  soci::session db(*this->_pool);
  soci::rowset<soci::row> rs =
      (db.prepare
       << R"SQL(SELECT ptype, v0, v1, v2, v3, v4, v5 FROM casbin_rule ORDER BY id ASC)SQL");
  for (auto it = rs.begin(); it != rs.end(); ++it) {
    soci::row const& row = *it;
    boost::tuple<std::string, std::string, std::string, std::string,
                 std::string, std::string, std::string>
        line;
    row >> line;
    // https://github.com/casbin/casbin-cpp/blob/master/casbin/persist/adapter.cpp#L28
    std::string key = line.get<0>();
    if (key.length() < 1) {
      spdlog::warn("bad ptype: {}", key);
      continue;
    }
    const std::string sec = key.substr(0, 1);

    if (model->m.find(sec) == model->m.end()) {
      model->m[sec] = ::casbin::AssertionMap();
    }
    model->m[sec].assertion_map[key]->policy.emplace(
        {line.get<1>(), line.get<2>(), line.get<3>(), line.get<4>(),
         line.get<5>()});
  }
}
void palm::casbin::PostgreSQLAdapter::SavePolicy(
    const std::shared_ptr<::casbin::Model>& model) {
  soci::session db(*this->_pool);
  {
    soci::transaction tr(db);

    clear_policies(db);

    for (std::unordered_map<std::string,
                            std::shared_ptr<::casbin::Assertion>>::iterator it =
             model->m["p"].assertion_map.begin();
         it != model->m["p"].assertion_map.end(); it++) {
      for (auto& rule : it->second->policy) {
        save_policy(db, it->first, rule);
      }
    }

    for (std::unordered_map<std::string,
                            std::shared_ptr<::casbin::Assertion>>::iterator it =
             model->m["g"].assertion_map.begin();
         it != model->m["g"].assertion_map.end(); it++) {
      for (auto& rule : it->second->policy) {
        save_policy(db, it->first, rule);
      }
    }

    tr.commit();
  }
}
void palm::casbin::PostgreSQLAdapter::AddPolicy(std::string sec,
                                                std::string p_type,
                                                std::vector<std::string> rule) {
  soci::session db(*this->_pool);
  {
    soci::transaction tr(db);
    save_policy(db, p_type, rule);
    tr.commit();
  }
}
void palm::casbin::PostgreSQLAdapter::RemovePolicy(
    std::string sec, std::string p_type, std::vector<std::string> rule) {
  soci::session db(*this->_pool);
  {
    soci::transaction tr(db);
    remove_policy(db, p_type, rule);
    tr.commit();
  }
}
void palm::casbin::PostgreSQLAdapter::RemoveFilteredPolicy(
    std::string sec, std::string ptype, int field_index,
    std::vector<std::string> field_values) {
  throw ::casbin::UnsupportedOperationException("not implemented");
}
bool palm::casbin::PostgreSQLAdapter::IsFiltered() {
  return this->_is_filtered;
}
bool palm::casbin::PostgreSQLAdapter::IsValid() {
  try {
    soci::session db(*this->_pool);
    std::string now;
    db << "SELECT CURRENT_TIMESTAMP", soci::into(now);
    return true;
  } catch (soci::soci_error& e) {
    spdlog::error("{}", e.get_error_message());
    return false;
  }
}
