#include "palm/rbac.hpp"

#include <boost/algorithm/string/join.hpp>

static void save_policy(soci::session& db, const std::string& p_type,
                        const std::vector<std::string>& rule) {
  switch (rule.size()) {
    case 2:
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1) VALUES(:ptype, :v0, :v1))SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1");
      break;
    case 3:
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2) VALUES(:ptype, :v0, :v1, :v2))SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2");
      break;
    case 4:
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3) VALUES(:ptype, :v0, :v1, :v2, :v3))SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3");
      break;
    case 5:
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3, v4) VALUES(:ptype, :v0, :v1, :v2, :v3, :v4))SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3"), soci::use(rule[4], "v4");
      break;
    case 6:
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3, v4, v5) VALUES(:ptype, :v0, :v1, :v2, :v3, :v4, :v5))SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3"), soci::use(rule[4], "v4"),
          soci::use(rule[5], "v5");
      break;
    default:
      spdlog::warn("ignore save policy rule:{} {}", p_type,
                   boost::algorithm::join(rule, ","));
      return;
  }
}
static void clear_policies(soci::session& db) {
  db << R"SQL(DELETE FROM casbin_rule)SQL";
}
static void remove_policy(soci::session& db, const std::string& p_type,
                          const std::vector<std::string>& rule) {
  switch (rule.size()) {
    case 2:
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2='' AND v3='' AND v4='' AND v5='')SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1");
      break;
    case 3:
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3='' AND v4='' AND v5='')SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2");
      break;
    case 4:
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4='' AND v5='')SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3");
      break;
    case 5:
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4=:v4 AND v5='')SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3"), soci::use(rule[4], "v4");
      break;
    case 6:
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4=:v4 AND v5=:v5)SQL",
          soci::use(p_type, "ptype"), soci::use(rule[0], "v0"),
          soci::use(rule[1], "v1"), soci::use(rule[2], "v2"),
          soci::use(rule[3], "v3"), soci::use(rule[4], "v4"),
          soci::use(rule[5], "v5");
      break;
    default:
      spdlog::warn("ignore remove policy rule:{} {}", p_type,
                   boost::algorithm::join(rule, ","));
      return;
  }
}

void palm::casbin::PostgreSQLAdapter::LoadPolicy(
    const std::shared_ptr<::casbin::Model>& model) {
  soci::session db(*this->_pool);
  soci::rowset<boost::tuple<std::string, std::string, std::string, std::string,
                            std::string, std::string, std::string>>
      rs =
          (db.prepare
           << R"SQL(SELECT ptype, v0, v1, v2, v3, v4, v5 FROM casbin_rule ORDER BY id ASC)SQL");
  for (auto it = rs.begin(); it != rs.end(); ++it) {
    std::string p_type, v0, v1, v2, v3, v4, v5;
    boost::tie(p_type, v0, v1, v2, v3, v4, v5) = *it;

    // https://github.com/casbin/casbin-cpp/blob/master/casbin/persist/adapter.cpp#L28

    if (!p_type.empty()) {
      const std::string sec = p_type.substr(0, 1);

      if (model->m.find(sec) == model->m.end()) {
        model->m[sec] = ::casbin::AssertionMap();
      }
      if (!v0.empty() && !v1.empty() && !v2.empty() && !v3.empty() &&
          !v4.empty() && !v5.empty()) {
        model->m[sec].assertion_map[p_type]->policy.emplace(
            {v0, v1, v2, v3, v4, v5});
        continue;
      }
      if (!v0.empty() && !v1.empty() && !v2.empty() && !v3.empty() &&
          !v4.empty() && v5.empty()) {
        model->m[sec].assertion_map[p_type]->policy.emplace(
            {v0, v1, v2, v3, v4});
        continue;
      }
      if (!v0.empty() && !v1.empty() && !v2.empty() && !v3.empty() &&
          v4.empty() && v5.empty()) {
        model->m[sec].assertion_map[p_type]->policy.emplace({v0, v1, v2, v3});
        continue;
      }
      if (!v0.empty() && !v1.empty() && !v2.empty() && v3.empty() &&
          v4.empty() && v5.empty()) {
        model->m[sec].assertion_map[p_type]->policy.emplace({v0, v1, v2});
        continue;
      }
      if (!v0.empty() && !v1.empty() && v2.empty() && v3.empty() &&
          v4.empty() && v5.empty()) {
        model->m[sec].assertion_map[p_type]->policy.emplace({v0, v1});
        continue;
      }
    }
    spdlog::warn("ignore casbin-rule: {},{},{},{},{},{},{}", p_type, v0, v1, v2,
                 v3, v4, v5);
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
