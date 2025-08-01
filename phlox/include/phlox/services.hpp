#include "monitoring.grpc.pb.h"
#include "palm/jwt.hpp"
#include "palm/search.hpp"
#include "palm/session.hpp"

namespace phlox {

class CurrentUser : public palm::Session {
 public:
  CurrentUser(grpc::ServerContext* context) : Session(context) {}
  std::optional<std::string> name(std::shared_ptr<palm::Jwt> jwt) {
    if (this->_token) {
      const auto& [jid, kid, sub, pay] =
          jwt->verify(this->_token.value(), ISSUER, WEB_AUDIENCE);
      return kid;
    }
    return std::nullopt;
  }

  inline static const std::string ISSUER = "phlox";
  inline static const std::string WEB_AUDIENCE = "web";
};

namespace monitoring {
namespace services {
class SiteServiceImpl final : public palm::monitoring::v1::Site::Service {
 public:
  explicit SiteServiceImpl(std::shared_ptr<palm::Jwt> jwt,
                           std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Layout(grpc::ServerContext* context,
                      const palm::monitoring::v1::SiteLayoutRequest* request,
                      palm::monitoring::v1::SiteLayoutResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
class FileSystemServiceImpl final
    : public palm::monitoring::v1::FileSystem::Service {
 public:
  explicit FileSystemServiceImpl(
      std::shared_ptr<palm::Jwt> jwt,
      std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Logs(
      grpc::ServerContext* context,
      const palm::monitoring::v1::FileSystemLogsRequest* request,
      palm::monitoring::v1::FileSystemLogsResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
class SystemdServiceImpl final : public palm::monitoring::v1::Systemd::Service {
 public:
  explicit SystemdServiceImpl(std::shared_ptr<palm::Jwt> jwt,
                              std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Journal(
      grpc::ServerContext* context,
      const palm::monitoring::v1::SystemdJournalRequest* request,
      palm::monitoring::v1::SystemdJournalResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
class PodmanServiceImpl final : public palm::monitoring::v1::Podman::Service {
 public:
  explicit PodmanServiceImpl(std::shared_ptr<palm::Jwt> jwt,
                             std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Logs(grpc::ServerContext* context,
                    const palm::monitoring::v1::PodmanQueryRequest* request,
                    palm::monitoring::v1::PodmanLogsResponse* reply) override;
  grpc::Status Containers(
      grpc::ServerContext* context,
      const palm::monitoring::v1::PodmanQueryRequest* request,
      palm::monitoring::v1::PodmanContainersResponse* reply) override;
  grpc::Status Statistics(
      grpc::ServerContext* context,
      const palm::monitoring::v1::PodmanQueryRequest* request,
      palm::monitoring::v1::PodmanStatisticsResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
}  // namespace services
}  // namespace monitoring
}  // namespace phlox
