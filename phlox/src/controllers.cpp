#include "palm/controllers.hpp"
#include "palm/captcha.hpp"
#include "palm/crypto.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

void palm::mount(httplib::Server& server, std::shared_ptr<palm::Jwt> jwt,
                 std::shared_ptr<grpc::Channel> channel) {
  server.Get("/api/layout", [ch = channel](const httplib::Request& request,
                                           httplib::Response& response) {
    auto stub = palm::monitoring::v1::Site::NewStub(ch);
    grpc::ClientContext ctx;
    palm::Session::init(request, &ctx);
    palm::monitoring::v1::SiteLayoutRequest req;
    palm::monitoring::v1::SiteLayoutResponse res;
    auto status = stub->Layout(&ctx, req, &res);
    if (!status.ok()) {
      palm::http::abort(response, status);
      return;
    }
    palm::http::json(response, res);
  });

  server.Get("/captcha.png", [](const httplib::Request& request,
                                httplib::Response& response) {
    // const auto file = std::filesystem::temp_directory_path() /
    //                   std::format("{}.png", palm::uuid());
    // response.set_file_content(file,
    // palm::http::content_type::IMAGE_PNG);
    const std::string code = palm::random::alphanumeric(6);
    const auto buf = palm::captcha::png(code, 32);
    if (buf.size() == 0) {
      palm::http::abort(response, "failed to generate a captcha picture");
      return;
    }
    response.set_content(reinterpret_cast<const char*>(buf.data()), buf.size(),
                         palm::http::content_type::IMAGE_PNG);
  });
}
