#include "palm/theme.hpp"

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
