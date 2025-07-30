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

  // ----------------------------------------------------------------------------
  server.Get("/api/systemd/by-host/:host",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto hostname = request.path_params.at("name");
               auto stub = palm::monitoring::v1::Systemd::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::SystemdJournalRequest req;
               {
                 req.set_host(hostname);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::SystemdJournalResponse res;
               auto status = stub->Journal(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/systemd/by-name/:name",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto service_name = request.path_params.at("name");
               auto stub = palm::monitoring::v1::Systemd::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::SystemdJournalRequest req;
               {
                 req.set_name(service_name);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::SystemdJournalResponse res;
               auto status = stub->Journal(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/systemd/journal",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::Systemd::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::SystemdJournalRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::SystemdJournalResponse res;
               auto status = stub->Journal(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  // ----------------------------------------------------------------------------
  server.Get("/api/podman/logs/by-service/:name",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto container_name = request.path_params.at("name");

               auto stub = palm::monitoring::v1::Podman::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.set_name(container_name);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::PodmanLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/podman/logs/by-id/:id",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto container_id = request.path_params.at("id");

               auto stub = palm::monitoring::v1::Podman::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.set_id(container_id);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::PodmanLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/podman/logs/by-host/:host",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto hostname = request.path_params.at("host");

               auto stub = palm::monitoring::v1::Podman::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.set_host(hostname);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::PodmanLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/podman/logs", [ch = channel](const httplib::Request& request,
                                                httplib::Response& response) {
    auto stub = palm::monitoring::v1::Podman::NewStub(ch);
    grpc::ClientContext ctx;
    palm::Session::init(request, &ctx);
    palm::monitoring::v1::PodmanQueryRequest req;
    {
      req.mutable_all();
      palm::page(request, req.mutable_page());
    }
    palm::monitoring::v1::PodmanLogsResponse res;
    auto status = stub->Logs(&ctx, req, &res);
    if (!status.ok()) {
      palm::http::abort(response, status);
      return;
    }
    palm::http::json(response, res);
  });

  // ----------------------------------------------------------------------------
  server.Get("/api/podman/containers",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::Podman::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::PodmanContainersResponse res;
               auto status = stub->Containers(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  // ----------------------------------------------------------------------------
  server.Get("/api/podman/statistics",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::Podman::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::PodmanStatisticsResponse res;
               auto status = stub->Statistics(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  // ----------------------------------------------------------------------------

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
