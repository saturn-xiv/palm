#pragma once

#include <algorithm>
#include <chrono>
#include <condition_variable>
#include <ctime>
#include <deque>
#include <filesystem>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <limits>
#include <list>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <queue>
#include <sstream>
#include <stdexcept>
#include <streambuf>
#include <string>
#include <thread>
#include <tuple>
#include <unordered_set>
#include <utility>
#include <variant>
#include <vector>

#include <Poco/LogStream.h>
#include <Poco/Logger.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <inja/inja.hpp>
#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS true
#include <toml.hpp>

#include "palm/version.hpp"

namespace nlohmann {

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(nlohmann::json &j, const std::optional<T> &opt) {
    if (opt == std::nullopt) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const nlohmann::json &j, std::optional<T> &opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.get<T>();
    }
  }
};

template <>
struct adl_serializer<std::filesystem::path> {
  static void to_json(nlohmann::json &j, const std::filesystem::path &opt) {
    j = opt.string();
  }

  static void from_json(const nlohmann::json &j, std::filesystem::path &opt) {
    opt = j.get<std::string>();
  }
};

template <typename T>
struct adl_serializer<std::shared_ptr<T>> {
  static void to_json(json &j, const std::shared_ptr<T> &opt) {
    if (opt.get()) {
      j = *opt;
    } else {
      j = nullptr;
    }
  }
};

}  // namespace nlohmann

// namespace palm {

// // #define PALM_BOOST_PTIME_ISO8601_FORMAT "%Y-%m-%dT%H:%M:%S%f"
// // #define PALM_LOCALE_ENGLISH "en_US.UTF-8"

// // #define PALM_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
// // #define PALM_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
// // #define PALM_PLAIN_TEXT_UTF8 "text/plain; charset=UTF-8"

// }  // namespace palm
