#pragma once

#include <format>

#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>
#include <spdlog/spdlog.h>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {
class GRpcClient {
 public:
  GRpcClient(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(8080)) {}
  GRpcClient(const std::string& host = "127.0.0.1", uint16_t port = 8080)
      : _host(host), _port(port) {}

  inline std::shared_ptr<grpc::Channel> open() {
    const std::string host = std::format("{}:{}", this->_host, this->_port);
    spdlog::debug("connect to gRpc server {}", host);
    return grpc::CreateChannel(host, grpc::InsecureChannelCredentials());
  }

 private:
  std::string _host;
  uint16_t _port;
};
}  // namespace palm
