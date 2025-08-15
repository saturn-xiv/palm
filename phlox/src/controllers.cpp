#include "phlox/controllers.hpp"
#include "palm/captcha.hpp"
#include "palm/crypto.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
podman_logs(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
            const std::string& req_) {
  auto stub = palm::monitoring::v1::Podman::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::PodmanLogsResponse> res =
      std::make_shared<palm::monitoring::v1::PodmanLogsResponse>();
  auto status = stub->Logs(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
podman_containers(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                  const std::string& req_) {
  auto stub = palm::monitoring::v1::Podman::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::PodmanContainersResponse> res =
      std::make_shared<palm::monitoring::v1::PodmanContainersResponse>();
  auto status = stub->Containers(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
podman_statistics(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                  const std::string& req_) {
  auto stub = palm::monitoring::v1::Podman::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::PodmanStatisticsResponse> res =
      std::make_shared<palm::monitoring::v1::PodmanStatisticsResponse>();
  auto status = stub->Statistics(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
docker_logs(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
            const std::string& req_) {
  auto stub = palm::monitoring::v1::Docker::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::PodmanLogsResponse> res =
      std::make_shared<palm::monitoring::v1::PodmanLogsResponse>();
  auto status = stub->Logs(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
docker_containers(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                  const std::string& req_) {
  auto stub = palm::monitoring::v1::Docker::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::DockerContainersResponse> res =
      std::make_shared<palm::monitoring::v1::DockerContainersResponse>();
  auto status = stub->Containers(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
docker_statistics(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                  const std::string& req_) {
  auto stub = palm::monitoring::v1::Docker::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::DockerStatisticsResponse> res =
      std::make_shared<palm::monitoring::v1::DockerStatisticsResponse>();
  auto status = stub->Statistics(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
filesystem_logs(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                const std::string& req_) {
  auto stub = palm::monitoring::v1::FileSystem::NewStub(ch);
  palm::monitoring::v1::FileSystemLogsRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::FileSystemLogsResponse> res =
      std::make_shared<palm::monitoring::v1::FileSystemLogsResponse>();
  auto status = stub->Logs(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
}

static std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>>
systemd_journal(std::shared_ptr<grpc::Channel> ch, grpc::ClientContext& ctx,
                const std::string& req_) {
  auto stub = palm::monitoring::v1::Docker::NewStub(ch);
  palm::monitoring::v1::PodmanQueryRequest req;
  {
    const auto status = palm::from_json(req_, &req);
    if (!status.ok()) {
      std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
          std::make_pair(status, nullptr);
      return it;
    }
  }
  std::shared_ptr<palm::monitoring::v1::DockerStatisticsResponse> res =
      std::make_shared<palm::monitoring::v1::DockerStatisticsResponse>();
  auto status = stub->Statistics(&ctx, req, res.get());
  std::pair<grpc::Status, std::shared_ptr<google::protobuf::Message>> it =
      std::make_pair(status,
                     std::dynamic_pointer_cast<google::protobuf::Message>(res));
  return it;
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

static void mount_fs_watcher(httplib::Server& server,
                             std::shared_ptr<palm::Jwt> jwt,
                             std::shared_ptr<grpc::Channel> channel) {
  server.Get("/api/filesystem/logs/by-host/:host",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto hostname = request.path_params.at("name");
               auto stub = palm::monitoring::v1::FileSystem::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::FileSystemLogsRequest req;
               {
                 req.set_host(hostname);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::FileSystemLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get(R"P(/api/filesystem/logs/by-file/.*)P",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               const auto file = request.matches[1];
               auto stub = palm::monitoring::v1::FileSystem::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::FileSystemLogsRequest req;
               {
                 req.set_file(file);
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::FileSystemLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
               if (!status.ok()) {
                 palm::http::abort(response, status);
                 return;
               }
               palm::http::json(response, res);
             });
  server.Get("/api/filesystem/logs",
             [ch = channel](const httplib::Request& request,
                            httplib::Response& response) {
               auto stub = palm::monitoring::v1::FileSystem::NewStub(ch);
               grpc::ClientContext ctx;
               palm::Session::init(request, &ctx);
               palm::monitoring::v1::FileSystemLogsRequest req;
               {
                 req.mutable_all();
                 palm::page(request, req.mutable_page());
               }
               palm::monitoring::v1::FileSystemLogsResponse res;
               auto status = stub->Logs(&ctx, req, &res);
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

  // mount_podman(server, jwt, channel);
  // mount_docker(server, jwt, channel);
  mount_systemd_journal(server, jwt, channel);
  mount_fs_watcher(server, jwt, channel);

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
