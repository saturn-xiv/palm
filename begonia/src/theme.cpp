#include "palm/theme.hpp"

#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/date_time/posix_time/posix_time_io.hpp>

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

std::pair<uint32_t, uint32_t> palm::paginate(uint32_t total, uint32_t index,
                                             uint32_t size) {
  const int MIN_SIZE = 10, MAX_SIZE = (1 << 12);

  if (size < MIN_SIZE) {
    size = MIN_SIZE;
  }
  if (size > MAX_SIZE) {
    size = MAX_SIZE;
  }
  if (index < 1) {
    index = 1;
  }
  if (total <= size) {
    return {1, size};
  }
  if (total < index * size) {
    if (total % size == 0) {
      return {total / size, size};
    }
    return {(total / size) + 1, size};
  }
  return {index, size};
}

void palm::http::mount(httplib::Server& server, const std::string& path,
                       std::shared_ptr<grpc::Channel> channel,
                       const std::set<palm::http::GRpcHandler>& handlers) {
  server.Post(std::format("{}/:package/:service/:method", path),
              [ch = channel, hnd = handlers](const httplib::Request& req,
                                             httplib::Response& res) mutable {
                const auto pkg = req.path_params.at("package");
                const auto srv = req.path_params.at("service");
                const auto mth = req.path_params.at("method");
                grpc::ClientContext ctx;
                palm::Session::init(req, &ctx);
                for (auto& h : hnd) {
                  if (h.package == pkg && h.service == srv && h.method == mth) {
                    const auto& [sts, msg] = h.handler(ch, ctx, req.body);
                    if (sts.ok()) {
                      if (msg) {
                        palm::http::json(res, *msg);
                      } else {
                        palm::http::text(res);
                      }
                    } else {
                      palm::http::abort(res, sts);
                    }
                    return;
                  }
                }
                res.status = httplib::StatusCode::NotFound_404;
              });
}
