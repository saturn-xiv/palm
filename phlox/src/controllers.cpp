#include "phlox/controllers.hpp"
#include "palm/captcha.hpp"
#include "palm/crypto.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

static void mount_podman(httplib::Server& server,
                         std::shared_ptr<palm::Jwt> jwt,
                         std::shared_ptr<grpc::Channel> channel) {
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
}

static void mount_docker(httplib::Server& server,
                         std::shared_ptr<palm::Jwt> jwt,
                         std::shared_ptr<grpc::Channel> channel) {
  server.Get("/api/docker/logs/by-service/:name",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto container_name = request.path_params.at("name");

               auto stub = palm::monitoring::v1::Docker::NewStub(ch);
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
  server.Get("/api/docker/logs/by-id/:id",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto container_id = request.path_params.at("id");

               auto stub = palm::monitoring::v1::Docker::NewStub(ch);
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
  server.Get("/api/docker/logs/by-host/:host",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto hostname = request.path_params.at("host");

               auto stub = palm::monitoring::v1::Docker::NewStub(ch);
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
  server.Get("/api/docker/logs", [ch = channel](const httplib::Request& request,
                                                httplib::Response& response) {
    auto stub = palm::monitoring::v1::Docker::NewStub(ch);
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
  server.Get("/api/docker/containers",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::Docker::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::DockerContainersResponse res;
               auto status = stub->Containers(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  // ----------------------------------------------------------------------------
  server.Get("/api/docker/statistics",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::Docker::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::PodmanQueryRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::DockerStatisticsResponse res;
               auto status = stub->Statistics(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
}

static void mount_systemd_journal(httplib::Server& server,
                                  std::shared_ptr<palm::Jwt> jwt,
                                  std::shared_ptr<grpc::Channel> channel) {
  server.Get("/api/systemd/journal/by-host/:host",
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
  server.Get("/api/systemd/journal/by-name/:name",
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
}

void phlox::mount(httplib::Server& server, std::shared_ptr<palm::Jwt> jwt,
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

  mount_podman(server, jwt, channel);
  mount_docker(server, jwt, channel);
  mount_systemd_journal(server, jwt, channel);

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
