#pragma once

#include <algorithm>
#include <chrono>
#include <climits>
#include <condition_variable>
#include <cstdint>
#include <cstdlib>
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
#include <random>
#include <set>
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

#define BOOST_LOCALE_HIDE_AUTO_PTR
#include <boost/locale.hpp>
#include <boost/log/trivial.hpp>
#include <boost/optional.hpp>

#include <date/date.h>
#include <grpcpp/grpcpp.h>
#include <httplib.h>
#include <sw/redis++/redis++.h>
#include <inja/inja.hpp>
#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS true
#include <toml.hpp>

#define SOCI_USE_BOOST
#include <soci/soci.h>

#include "palm/version.hpp"

// https://www.w3docs.com/snippets/javascript/the-right-json-date-format.html
#define PALM_STDTM_ISO8601_FORMAT "%FT%TZ"
#define PALM_BOOST_PTIME_ISO8601_FORMAT "%Y-%m-%dT%H:%M:%S%f"

#define PALM_LOCALE_ENGLISH "en_US.UTF-8"

#define PALM_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PALM_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PALM_PLAIN_TEXT_UTF8 "text/plain; charset=UTF-8"

namespace palm {

// https://www.boost.org/doc/libs/1_74_0/libs/locale/doc/html/messages_formatting.html
inline std::string tr(const std::string &lang, const std::string &code) {
  return boost::locale::translate(code).str(lang);
}

namespace iso8601 {
inline static const std::string format = "%Y-%m-%dT%H:%M:%SZ";
inline std::string to(const std::tm *t) {
  std::stringstream ss;
  ss.exceptions(std::ios::failbit);
  ss << std::put_time(t, format.c_str());
  return ss.str();
}

inline std::tm from(const std::string &s) {
  std::tm t = {};
  std::istringstream ss(s);
  ss.imbue(std::locale("en_US.utf-8"));
  ss.exceptions(std::ios::failbit);
  ss >> std::get_time(&t, format.c_str());
  if (ss.fail()) {
    BOOST_LOG_TRIVIAL(error) << "parse time [" << format << "] " << s;
  }
  return t;
}
}  // namespace iso8601
}  // namespace palm

namespace nlohmann {

template <typename T>
struct adl_serializer<boost::optional<T>> {
  static void to_json(nlohmann::json &j, const boost::optional<T> &opt) {
    if (opt) {
      j = *opt;
    } else {
      j = nullptr;
    }
  }

  static void from_json(const nlohmann::json &j, boost::optional<T> &opt) {
    if (j.is_null()) {
      opt = boost::none;
    } else {
      opt = j.get<T>();
    }
  }
};

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(nlohmann::json &j, const std::optional<T> &opt) {
    if (opt) {
      j = *opt;
    } else {
      j = nullptr;
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

template <>
struct adl_serializer<std::chrono::years> {
  static void to_json(nlohmann::json &j, const std::chrono::years &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::years &opt) {
    opt = std::chrono::years(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::months> {
  static void to_json(nlohmann::json &j, const std::chrono::months &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::months &opt) {
    opt = std::chrono::months(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::weeks> {
  static void to_json(nlohmann::json &j, const std::chrono::weeks &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::weeks &opt) {
    opt = std::chrono::weeks(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::days> {
  static void to_json(nlohmann::json &j, const std::chrono::days &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::days &opt) {
    opt = std::chrono::days(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::hours> {
  static void to_json(nlohmann::json &j, const std::chrono::hours &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::hours &opt) {
    opt = std::chrono::hours(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::minutes> {
  static void to_json(nlohmann::json &j, const std::chrono::minutes &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::minutes &opt) {
    opt = std::chrono::minutes(j.get<long>());
  }
};

template <>
struct adl_serializer<std::chrono::seconds> {
  static void to_json(nlohmann::json &j, const std::chrono::seconds &opt) {
    j = opt.count();
  }

  static void from_json(const nlohmann::json &j, std::chrono::seconds &opt) {
    opt = std::chrono::seconds(j.get<long>());
  }
};

template <>
struct adl_serializer<std::tm> {
  static void to_json(nlohmann::json &j, const std::tm &opt) {
    j = palm::iso8601::to(&opt);
  }

  static void from_json(const nlohmann::json &j, std::tm &opt) {
    const std::string it = j.get<std::string>();
    opt = palm::iso8601::from(it);
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
