#include "lavender/portal.hpp"

#include <boost/current_function.hpp>

void lavender::portal::mount(httplib::Server& server, lavender::GrpcClient& rpc,
                             palm::Theme& theme, std::shared_ptr<palm::Jwt> jwt,
                             std::shared_ptr<palm::Minio> s3) {
  spdlog::debug("{}", BOOST_CURRENT_FUNCTION);
  //   TODO
  server.Get("/", [&](const auto& req, auto& res) {
    nlohmann::json data;
    data["title"] = "hi";
    const auto body = theme.render("home.html", data);
    res.set_content(body, palm::http::content_type::TEXT_HTML_UTF8);
  });
}
