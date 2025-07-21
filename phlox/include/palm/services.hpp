#include "monitoring.grpc.pb.h"
#include "palm/jwt.hpp"
#include "palm/search.hpp"

namespace palm {
namespace monitoring {
namespace services {
class SiteServiceImpl final : public v1::Site::Service {
 public:
  explicit SiteServiceImpl(std::shared_ptr<palm::Jwt> jwt,
                           std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Layout(grpc::ServerContext* context,
                      const v1::SiteLayoutRequest* request,
                      v1::SiteLayoutResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
class FileSystemServiceImpl final : public v1::FileSystem::Service {
 public:
  explicit FileSystemServiceImpl(
      std::shared_ptr<palm::Jwt> jwt,
      std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Logs(grpc::ServerContext* context,
                    const v1::FileSystemLogsRequest* request,
                    v1::FileSystemLogsResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
class PodmanServiceImpl final : public v1::Podman::Service {
 public:
  explicit PodmanServiceImpl(std::shared_ptr<palm::Jwt> jwt,
                             std::shared_ptr<palm::opensearch::Client> search)
      : _jwt(jwt), _search(search) {}
  grpc::Status Logs(grpc::ServerContext* context,
                    const v1::PodmanQueryRequest* request,
                    v1::PodmanLogsResponse* reply) override;
  grpc::Status Containers(grpc::ServerContext* context,
                          const v1::PodmanQueryRequest* request,
                          v1::PodmanContainersResponse* reply) override;
  grpc::Status Statistics(grpc::ServerContext* context,
                          const v1::PodmanQueryRequest* request,
                          v1::PodmanStatisticsResponse* reply) override;

 private:
  std::shared_ptr<palm::opensearch::Client> _search;
  std::shared_ptr<palm::Jwt> _jwt;
};
}  // namespace services
}  // namespace monitoring
}  // namespace palm
