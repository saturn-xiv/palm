#pragma once

#include "palm/orm.hpp"
#include "router.grpc.pb.h"

namespace bamboo {

namespace dao {

namespace host {
struct Item {
  uint32_t id;
  uint32_t user_id;
  std::string mac;
  std::string ip;
  std::string name;
  std::string vendor;
  bool fixed;
  std::string description;
  std::optional<std::tm> deleted_at;
  uint32_t version;
  std::tm updated_at;
  std::tm created_at;
};
boost::fusion::vector<Item> all(soci::session& db);
void save(soci::session& db, const std::string& mac, const std::string& name,
          const std::string& ip, const std::string& vendor);
void set_description(soci::session& db, uint32_t id,
                     const std::string& description);
void enable(soci::session& db, uint32_t id, bool ok=true);                     
}  // namespace host

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

namespace soci {
template <>
struct type_conversion<bamboo::dao::host::Item> {
  typedef values base_type;

  static void from_base(values const& v, indicator /* ind */,
                        bamboo::dao::host::Item& p) {
    p.id = v.get<uint32_t>("id");
    p.user_id = v.get<uint32_t>("user_id");
    p.mac = v.get<std::string>("mac");
    p.ip = v.get<std::string>("ip");
    p.name = v.get<std::string>("name");
    p.vendor = v.get<std::string>("vendor");
    p.fixed = v.get<int>("fixed") == 1;
    p.description = v.get<std::string>("description");
    if (v.get_indicator("deleted_at") == soci::i_null) {
      p.deleted_at = std::nullopt;
    } else {
      p.deleted_at = v.get<std::tm>("deleted_at");
    }
    p.version = v.get<uint32_t>("version");
    p.updated_at = v.get<std::tm>("updated_at");
    p.created_at = v.get<std::tm>("created_at");
  }
};
}  // namespace soci
