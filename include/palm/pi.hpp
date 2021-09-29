// #pragma once

// #include "palm/audio.hpp"
// #include "palm/crypto.hpp"
// #include "palm/orm.hpp"
// #include "palm/queue.hpp"
// #include "palm/theme.hpp"
// #include "palm/tty.hpp"
// #include "pi.grpc.pb.h"

// namespace palm {
// namespace pi {
// inline static const std::filesystem::path ROOT = "tmp";

// class Service final : public palm::pi::v1::Pi::Service {
//  public:
//   Service(std::shared_ptr<Jwt> jwt) : jwt(jwt) {}
//   ~Service() {}
//   grpc::Status SignIn(grpc::ServerContext* context,
//                       const palm::pi::v1::SignInRequest* request,
//                       palm::pi::v1::SignInResponse* response) override;
//   grpc::Status SignOut(grpc::ServerContext* context,
//                        const google::protobuf::Empty* request,
//                        google::protobuf::Empty* response) override;
//   grpc::Status GetUser(grpc::ServerContext* context,
//                        const google::protobuf::Empty* request,
//                        palm::pi::v1::Administrator* response) override;
//   grpc::Status SetUser(grpc::ServerContext* context,
//                        const palm::pi::v1::Administrator* request,
//                        google::protobuf::Empty* response) override;
//   grpc::Status Logs(grpc::ServerContext* context,
//                     const google::protobuf::Empty* request,
//                     palm::pi::v1::LogsResponse* response) override;
//   grpc::Status Status(grpc::ServerContext* context,
//                       const google::protobuf::Empty* request,
//                       palm::pi::v1::StatusResponse* response) override;
//   grpc::Status GetEther(grpc::ServerContext* context,
//                         const google::protobuf::Empty* request,
//                         palm::pi::v1::Ether* response) override;
//   grpc::Status SetEther(grpc::ServerContext* context,
//                         const palm::pi::v1::Ether* request,
//                         google::protobuf::Empty* response) override;
//   grpc::Status GetWifi(grpc::ServerContext* context,
//                        const google::protobuf::Empty* request,
//                        palm::pi::v1::Wifi* response) override;
//   grpc::Status SetWifi(grpc::ServerContext* context,
//                        const palm::pi::v1::Wifi* request,
//                        google::protobuf::Empty* response) override;
//   grpc::Status Ntp(grpc::ServerContext* context,
//                    const google::protobuf::Empty* request,
//                    palm::pi::v1::LogsResponse* response) override;
//   grpc::Status Dns(grpc::ServerContext* context,
//                    const google::protobuf::Empty* request,
//                    palm::pi::v1::LogsResponse* response) override;
//   grpc::Status GetVpn(grpc::ServerContext* context,
//                       const google::protobuf::Empty* request,
//                       palm::pi::v1::VpnProfile* response) override;
//   grpc::Status SetVpn(grpc::ServerContext* context,
//                       const palm::pi::v1::VpnProfile* request,
//                       google::protobuf::Empty* response) override;

//   grpc::Status GetNtp(grpc::ServerContext* context,
//                       const google::protobuf::Empty* request,
//                       palm::pi::v1::NtpProfile* response) override;
//   grpc::Status SetNtp(grpc::ServerContext* context,
//                       const palm::pi::v1::NtpProfile* request,
//                       google::protobuf::Empty* response) override;
//   grpc::Status Reboot(grpc::ServerContext* context,
//                       const google::protobuf::Empty* request,
//                       google::protobuf::Empty* response) override;
//   grpc::Status Reset(grpc::ServerContext* context,
//                      const google::protobuf::Empty* request,
//                      google::protobuf::Empty* response) override;

//  private:
//   std::shared_ptr<palm::Jwt> jwt;
// };

// }  // namespace pi
// }  // namespace palm
