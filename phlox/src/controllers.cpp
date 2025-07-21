#include "palm/controllers.hpp"
#include "palm/theme.hpp"

void palm::mount(httplib::Server& server, std::shared_ptr<palm::Jwt> jwt,
                 std::shared_ptr<grpc::Channel> channel) {
  server.Get("/api/layout",
             [&](const httplib::Request& request, httplib::Response& response) {
               auto stub = palm::monitoring::v1::Site::NewStub(channel);
               grpc::ClientContext ctx;
               palm::monitoring::v1::SiteLayoutRequest req;
               palm::monitoring::v1::SiteLayoutResponse res;
               auto status = stub->Layout(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
}
