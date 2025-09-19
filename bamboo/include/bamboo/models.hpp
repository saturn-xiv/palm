#pragma once

#include "palm/orm.hpp"
#include "router.grpc.pb.h"

namespace bamboo {

namespace dao {

namespace host {
void save(soci::session& db, const std::string& mac, const std::string& name,
          const std::string& ip, const std::string& vendor);
}

namespace administrator {
void save(soci::session& db, const std::string& name,
          const std::string& password);
bool auth(soci::session& db, const std::string& name,
          const std::string& password);

}  // namespace administrator

void set(soci::session& db, const std::string& key,
         const std::vector<uint8_t>& value);
std::optional<std::vector<uint8_t>> get(soci::session& db,
                                        const std::string& key);
void set(soci::session& db, const std::vector<uint8_t>& secret,
         const std::string& key, const std::vector<uint8_t>& value);
std::optional<std::vector<uint8_t>> get(soci::session& db,
                                        const std::vector<uint8_t>& secret,
                                        const std::string& key);
inline void set(soci::session& db, const std::string& key,
                const google::protobuf::Message& value) {
  size_t len = value.ByteSizeLong();
  std::vector<uint8_t> buf(len);
  if (!value.SerializeToArray(buf.data(), len)) {
    spdlog::error("failed to serial protobuf message");
    return;
  }
  bamboo::dao::set(db, key, buf);
}
inline bool get(soci::session& db, const std::string& key,
                google::protobuf::Message* value) {
  const auto buf = bamboo::dao::get(db, key);
  if (!buf) {
    spdlog::error("empty record");
    return false;
  }
  if (!value->ParseFromArray(buf->data(), buf->size())) {
    spdlog::error("failed to parse protobuf message");
    return false;
  }
  return true;
}
}  // namespace dao
}  // namespace bamboo
