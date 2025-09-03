#pragma once

#include "bamboo/models.hpp"
#include "palm/crypto.hpp"
#include "palm/jwt.hpp"
#include "palm/session.hpp"
#include "palm/theme.hpp"
#include "router.grpc.pb.h"

namespace bamboo {

namespace services {
class UserServiceImpl final : public palm::router::v1::User::Service {
 public:
  UserServiceImpl(std::shared_ptr<soci::session> db,
                  std::shared_ptr<palm::Aes> aes,
                  std::shared_ptr<palm::Jwt> jwt)
      : _db(db), _aes(aes), _jwt(jwt) {}

  grpc::Status Index(grpc::ServerContext* context,
                     const google::protobuf::Empty* request,
                     palm::router::v1::UserIndexResponse* reply) override;
  grpc::Status Create(grpc::ServerContext* context,
                      const palm::router::v1::UserCreateRequest* request,
                      google::protobuf::Empty* reply) override;
  grpc::Status SetRealName(
      grpc::ServerContext* context,
      const palm::router::v1::UserSetRealNameRequest* request,
      google::protobuf::Empty* reply) override;
  grpc::Status SetContact(
      grpc::ServerContext* context,
      const palm::router::v1::UserSetContactRequest* request,
      google::protobuf::Empty* reply) override;
  grpc::Status SetWifi(grpc::ServerContext* context,
                       const palm::router::v1::UserSetWifiRequest* request,
                       google::protobuf::Empty* reply) override;

 private:
  std::shared_ptr<soci::session> _db;
  std::shared_ptr<palm::Aes> _aes;
  std::shared_ptr<palm::Jwt> _jwt;
};
class RouterServiceImpl final : public palm::router::v1::Router::Service {
 public:
  RouterServiceImpl(std::shared_ptr<soci::session> db,

                    std::shared_ptr<palm::Jwt> jwt)
      : _db(db), _jwt(jwt) {}

  grpc::Status SetEthernet(
      grpc::ServerContext* context,
      const palm::router::v1::RouterIndexEthernetResponse_Item* request,
      google::protobuf::Empty* reply) override;
  grpc::Status IndexEthernet(
      grpc::ServerContext* context, const google::protobuf::Empty* request,
      palm::router::v1::RouterIndexEthernetResponse* reply) override;
  grpc::Status Reboot(grpc::ServerContext* context,
                      const google::protobuf::Empty* request,
                      google::protobuf::Empty* reply) override;
  grpc::Status Apply(grpc::ServerContext* context,
                     const google::protobuf::Empty* request,
                     google::protobuf::Empty* reply) override;
  grpc::Status FactoryReset(grpc::ServerContext* context,
                            const google::protobuf::Empty* request,
                            google::protobuf::Empty* reply) override;

 private:
  std::shared_ptr<soci::session> _db;
  std::shared_ptr<palm::Jwt> _jwt;
};
class AdministratorServiceImpl final
    : public palm::router::v1::Administrator::Service {
 public:
  AdministratorServiceImpl(std::shared_ptr<soci::session> db,
                           std::shared_ptr<palm::Aes> aes,
                           std::shared_ptr<palm::HMac> hmac,
                           std::shared_ptr<palm::Jwt> jwt)
      : _db(db), _aes(aes), _hmac(hmac), _jwt(jwt) {}

  grpc::Status SignIn(
      grpc::ServerContext* context,
      const palm::router::v1::AdministratorSignInRequest* request,
      palm::router::v1::AdministratorSignInResponse* reply) override;
  grpc::Status SignOut(grpc::ServerContext* context,
                       const google::protobuf::Empty* request,
                       google::protobuf::Empty* reply) override;
  grpc::Status SetPassword(
      grpc::ServerContext* context,
      const palm::router::v1::AdministratorSetPasswordRequest* request,
      google::protobuf::Empty* reply) override;

 private:
  std::shared_ptr<soci::session> _db;
  std::shared_ptr<palm::Aes> _aes;
  std::shared_ptr<palm::Jwt> _jwt;
  std::shared_ptr<palm::HMac> _hmac;
};
}  // namespace services
}  // namespace bamboo
