#pragma once

#include "palm/theme.hpp"

namespace palm {
namespace systemd {
namespace models {
namespace journal {
struct Message {
  std::optional<std::string> content;
};
// sudo journalctl --output json-pretty -n 20 -u nginx
struct Item {
  std::string _HOSTNAME;
  std::string _MACHINE_ID;
  std::string __SEQNUM;
  std::string __SEQNUM_ID;
  std::string __REALTIME_TIMESTAMP;
  Message MESSAGE;
  std::string UNIT;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Item, _HOSTNAME, _MACHINE_ID, __SEQNUM,
                                   __SEQNUM_ID, __REALTIME_TIMESTAMP, MESSAGE,
                                   UNIT)
}  // namespace journal

}  // namespace models

std::vector<models::journal::Item> logs(const std::string& service_name,
                                        bool user_scope, time_t since,
                                        time_t until);
}  // namespace systemd
// https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html#Parsing%20Timestamps
inline std::string epoch_to_journald_timestamp(time_t epoch_in_seconds) {
  std::tm* it = std::localtime(&epoch_in_seconds);
  char buf[32];
  strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", it);
  return buf;
}
}  // namespace palm

namespace nlohmann {
template <>
struct adl_serializer<palm::systemd::models::journal::Message> {
  static void to_json(json& j,
                      const palm::systemd::models::journal::Message& o) {
    if (o.content) {
      j = o.content.value();
    } else {
      j = nullptr;
    }
  }

  static void from_json(const json& j,
                        palm::systemd::models::journal::Message& o) {
    if (j.is_null()) {
      o.content = nullptr;
    } else if (j.is_string()) {
      auto s = j.template get<std::string>();
      boost::trim(s);
      if (s.empty()) {
        o.content = nullptr;
      } else {
        o.content = s;
      }
    } else if (j.is_binary()) {
      auto buf = j.template get<std::vector<uint8_t>>();
      std::string s(buf.begin(), buf.end());
      boost::trim(s);
      if (s.empty()) {
        o.content = nullptr;
      } else {
        o.content = s;
      }
    } else {
      spdlog::error("unknown message value {}", j.dump());
      o.content = nullptr;
    }
  }
};
}  // namespace nlohmann
