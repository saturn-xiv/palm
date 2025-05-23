#pragma once

#include "loquat/env.hpp"

// TODO
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#include <thrift/Thrift.h>
#pragma GCC diagnostic pop

#include "petunia/Aes.h"
#include "petunia/Health.h"
#include "petunia/Hmac.h"
#include "petunia/Jwt.h"

namespace loquat {

namespace application {

struct Ssl {
  Ssl(const std::string& cert_file, const std::string& key_file,
      const std::string& ca_file)
      : cert_file(cert_file), key_file(key_file), ca_file(ca_file) {}
  std::string cert_file;
  std::string key_file;
  std::string ca_file;
};

void launch_rpc_server(const uint16_t port, std::optional<Ssl> ssl);
void generate_systemd_config(const std::string& name, const uint16_t port);
}  // namespace application

class AesHandler final : public v1::AesIf {
 public:
  AesHandler() = default;

  void encrypt(std::string& code, const std::string& plain) override;
  void decrypt(std::string& plain, const std::string& code) override;
};

class HmacHandler final : public v1::HmacIf {
 public:
  HmacHandler() = default;

  void sign(std::string& code, const std::string& plain) override;
  void verify(const std::string& code, const std::string& plain) override;
};

class JwtHandler final : public v1::JwtIf {
 public:
  JwtHandler() = default;

  void sign(std::string& token,
            const loquat::v1::JwtSignRequest& request) override;
  void verify(loquat::v1::JwtVerfifyResponse& response,
              const std::string& token, const std::string& issuer,
              const std::string& audience) override;
};

class HealthHandler final : public v1::HealthIf {
 public:
  HealthHandler() = default;

  void check(std::map<std::string, std::string>& response) override;
};
}  // namespace loquat
