#include "palm/rbac.hpp"

#include <boost/algorithm/string/join.hpp>

static void save_policy(soci::session& db, const std::string& p_type,
                        const std::vector<std::string>& rule) {
  palm::casbin::Rule it;
  it.p_type = p_type;
  switch (rule.size()) {
    case 2:
      it.v0 = rule[0];
      it.v1 = rule[1];
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1) VALUES(:ptype, :v0, :v1))SQL",
          soci::use(it);
      break;
    case 3:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2) VALUES(:ptype, :v0, :v1, :v2))SQL",
          soci::use(it);
      break;
    case 4:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3) VALUES(:ptype, :v0, :v1, :v2, :v3))SQL",
          soci::use(it);
      break;
    case 5:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      it.v4 = rule[4];
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3, v4) VALUES(:ptype, :v0, :v1, :v2, :v3, :v4))SQL",
          soci::use(it);
      break;
    case 6:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      it.v4 = rule[4];
      it.v5 = rule[5];
      db << R"SQL(INSERT INTO casbin_rule(ptype, v0, v1, v2, v3, v4, v5) VALUES(:ptype, :v0, :v1, :v2, :v3, :v4, :v5))SQL",
          soci::use(it);
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
  palm::casbin::Rule it;
  it.p_type = p_type;
  switch (rule.size()) {
    case 2:
      it.v0 = rule[0];
      it.v1 = rule[1];
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2='' AND v3='' AND v4='' AND v5='')SQL",
          soci::use(it);
      break;
    case 3:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3='' AND v4='' AND v5='')SQL",
          soci::use(it);
      break;
    case 4:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4='' AND v5='')SQL",
          soci::use(it);
      break;
    case 5:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      it.v4 = rule[4];
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4=:v4 AND v5='')SQL",
          soci::use(it);
      break;
    case 6:
      it.v0 = rule[0];
      it.v1 = rule[1];
      it.v2 = rule[2];
      it.v3 = rule[3];
      it.v4 = rule[4];
      it.v5 = rule[5];
      db << R"SQL(DELETE FROM casbin_rule WHERE ptype=:ptype AND v0=:v0 AND v1=:v1 AND v2=:v2 AND v3=:v3 AND v4=:v4 AND v5=:v5)SQL",
          soci::use(it);
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
  // soci::rowset<boost::tuple<std::string, std::string, std::string,
  // std::string,
  //                           std::string, std::string, std::string>>
  //     rs =
  //         (db.prepare
  //          << R"SQL(SELECT ptype, v0, v1, v2, v3, v4, v5 FROM casbin_rule
  //          ORDER BY id ASC)SQL");
  soci::rowset<palm::casbin::Rule> rs =
      (db.prepare
       << R"SQL(SELECT id, ptype, v0, v1, v2, v3, v4, v5 FROM casbin_rule ORDER BY id ASC)SQL");
  for (auto it = rs.begin(); it != rs.end(); ++it) {
    // std::string p_type, v0, v1, v2, v3, v4, v5;
    // boost::tie(p_type, v0, v1, v2, v3, v4, v5) = *it;

    // https://github.com/casbin/casbin-cpp/blob/master/casbin/persist/adapter.cpp#L28
    if (!it->p_type.empty()) {
      const std::string sec = it->p_type.substr(0, 1);

      if (model->m.find(sec) == model->m.end()) {
        model->m[sec] = ::casbin::AssertionMap();
      }
      if (!it->v0.empty() && !it->v1.empty() && !it->v2.empty() &&
          !it->v3.empty() && !it->v4.empty() && !it->v5.empty()) {
        model->m[sec].assertion_map[it->p_type]->policy.emplace(
            {it->v0, it->v1, it->v2, it->v3, it->v4, it->v5});
        continue;
      }
      if (!it->v0.empty() && !it->v1.empty() && !it->v2.empty() &&
          !it->v3.empty() && !it->v4.empty() && it->v5.empty()) {
        model->m[sec].assertion_map[it->p_type]->policy.emplace(
            {it->v0, it->v1, it->v2, it->v3, it->v4});
        continue;
      }
      if (!it->v0.empty() && !it->v1.empty() && !it->v2.empty() &&
          !it->v3.empty() && it->v4.empty() && it->v5.empty()) {
        model->m[sec].assertion_map[it->p_type]->policy.emplace(
            {it->v0, it->v1, it->v2, it->v3});
        continue;
      }
      if (!it->v0.empty() && !it->v1.empty() && !it->v2.empty() &&
          it->v3.empty() && it->v4.empty() && it->v5.empty()) {
        model->m[sec].assertion_map[it->p_type]->policy.emplace(
            {it->v0, it->v1, it->v2});
        continue;
      }
      if (!it->v0.empty() && !it->v1.empty() && it->v2.empty() &&
          it->v3.empty() && it->v4.empty() && it->v5.empty()) {
        model->m[sec].assertion_map[it->p_type]->policy.emplace(
            {it->v0, it->v1});
        continue;
      }
    }
    spdlog::warn("ignore casbin-rule: {},{},{},{},{},{},{}", it->p_type, it->v0,
                 it->v1, it->v2, it->v3, it->v4, it->v5);
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
    std::tm now;
    db << "SELECT CURRENT_TIMESTAMP", soci::into(now);
    return true;
  } catch (soci::soci_error& e) {
    spdlog::error("{}", e.get_error_message());
    return false;
  }
}
