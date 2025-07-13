#include "palm/theme.hpp"

#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/date_time/posix_time/posix_time_io.hpp>

#include <spdlog/spdlog.h>

void palm::set_logger(httplib::Server& server) {
  server.set_logger([&](const auto& req, const auto& res) {
    std::stringstream params;
    for (auto const& [k, v] : req.params) {
      params << k << "=" << v << " ";
    }
    spdlog::info("{} {} {} {}", req.method, res.status, req.path, params.str());
  });
}

void palm::tm2ts(std::tm* time, google::protobuf::Timestamp* timestamp) {
  const time_t seconds = std::mktime(time);
  timestamp->set_seconds(static_cast<int64_t>(seconds));
  timestamp->set_nanos(0);
}

std::optional<std::string> palm::to_json(
    const google::protobuf::Message& message) {
  std::string buf;
  const auto status =
      google::protobuf::util::MessageToJsonString(message, &buf);
  if (status.ok()) {
    return buf;
  }
  spdlog::error("failed to serialize google message to json {}",
                status.message());
  return std::nullopt;
}

// https://protobuf.dev/reference/cpp/api-docs/google.protobuf.util.time_util/#TimeUtil.ToString.details
// https://en.cppreference.com/w/cpp/chrono/parse.html
// https://www.epochconverter.com/
void palm::str2ts(const std::string& time,
                  google::protobuf::Timestamp* timestamp) {
  std::istringstream ss{time};

  // -----------------
  // https://www.boost.org/doc/libs/1_88_0/doc/html/date_time/date_time_io.html#date_time.format_flags
  boost::posix_time::time_input_facet* facet =
      new boost::posix_time::time_input_facet("%Y-%m-%d %H:%M:%S%f%q");
  ss.imbue(std::locale(std::locale::classic(), facet));
  boost::posix_time::ptime pt(boost::posix_time::microsec_clock::local_time());
  ss >> pt;
  if (ss.fail()) {
    spdlog::error("failed to parse timestamp");
    return;
  }
  // spdlog::debug("boost::posix_time::ptime: {}",
  //               boost::posix_time::to_iso_extended_string(pt));
  const static boost::posix_time::ptime epoch(
      boost::gregorian::date(1970, 1, 1),
      boost::posix_time::time_duration(0, 0, 0, 0));
  boost::posix_time::time_duration diff = pt - epoch;
  // spdlog::debug("{}, {}, {}, {}", diff.total_seconds(),
  //               diff.total_milliseconds(), diff.total_microseconds(),
  //               diff.total_nanoseconds());
  timestamp->set_seconds(diff.total_seconds());
  timestamp->set_nanos(
      static_cast<int32_t>(diff.total_nanoseconds() % 1000000000));

  // -------------------
  // std::chrono::system_clock::time_point tp;
  // ss >> std::chrono::parse("%F %T.%f%z", tp);
  // if (ss.fail()) {
  //   spdlog::error("failed to parse timestamp");
  //   return;
  // }
  // auto ep = tp.time_since_epoch();
  // auto ms = std::chrono::duration_cast<std::chrono::microseconds>(ep);
  // timestamp->set_seconds(ms.count() / 1000000);
  // timestamp->set_nanos((ms.count() % 1000000) * 1000);

  // ------------------------------

  // boost::posix_time::ptime tp;
  // ss.imbue(std::locale(
  //     std::locale::classic(),
  //     new boost::posix_time::time_input_facet("%Y-%m-%d %H:%M:%S.%f")));

  // if (ss.fail()) {
  //   spdlog::error("failed to parse timestamp");
  //   return;
  // }

  // std::cout << "### " << tp << std::endl;
  // const static boost::posix_time::ptime epoch(
  //     boost::gregorian::date(1970, 1, 1),
  //     boost::posix_time::time_duration(0, 0, 0));
  // boost::posix_time::time_duration diff = tp - epoch;
  // timestamp->set_seconds(diff.total_seconds());
  // timestamp->set_nanos(diff.total_nanoseconds() % 1000000000);
}
