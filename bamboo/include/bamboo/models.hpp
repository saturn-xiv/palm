#pragma once

#include "palm/orm.hpp"

namespace bamboo {

namespace dao {

namespace host {
void save(soci::session& db, const std::string& mac, const std::string& name,
          const std::string& ip, const std::string& vendor);
}

namespace user {
void save(soci::session& db, const std::string& name,
          const std::string& password);
bool auth(soci::session& db, const std::string& name,
          const std::string& password);

}  // namespace user

void set(soci::session& db, const std::string& key,
         const std::vector<uint8_t>& value);
std::optional<std::vector<uint8_t>> get(soci::session& db,
                                        const std::string& key);
void set(soci::session& db, const std::vector<uint8_t>& secret,
         const std::string& key, const std::vector<uint8_t>& value);
std::optional<std::vector<uint8_t>> get(soci::session& db,
                                        const std::vector<uint8_t>& secret,
                                        const std::string& key);

}  // namespace dao
}  // namespace bamboo
