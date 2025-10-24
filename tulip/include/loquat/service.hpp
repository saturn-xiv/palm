#pragma once

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#include <thrift/Thrift.h>
#pragma GCC diagnostic pop

#include "Aes.h"
#include "HMac.h"
#include "Health.h"
#include "Jwt.h"

namespace loquat {

std::string thrift_version();

namespace application {

struct Ssl {
  Ssl(const std::string& cert_file, const std::string& key_file,
      const std::string& ca_file)
      : cert_file(cert_file), key_file(key_file), ca_file(ca_file) {}
  std::string cert_file;
  std::string key_file;
  std::string ca_file;
};

void launch_rpc_server(const std::string& name, const uint16_t port,
                       std::optional<Ssl> ssl, size_t threads);
void generate_systemd_config(const std::string& name, const uint16_t port);
}  // namespace application

class AesHandler final : public v1::AesIf {
 public:
  AesHandler(const std::string& name) : AesIf(), _name(name) {}

  void encrypt(std::string& code, const std::string& plain) override;
  void decrypt(std::string& plain, const std::string& code) override;

 private:
  std::string _name;
};

class HMacHandler final : public v1::HMacIf {
 public:
  HMacHandler(const std::string& name) : HMacIf(), _name(name) {}

  void sign(std::string& code, const std::string& plain) override;
  void verify(const std::string& code, const std::string& plain) override;

 private:
  std::string _name;
};

class JwtHandler final : public v1::JwtIf {
 public:
  JwtHandler(const std::string& name) : JwtIf(), _name(name) {}

  void sign(std::string& token,
            const loquat::v1::JwtSignRequest& request) override;
  void verify(loquat::v1::JwtVerifyResponse& response, const std::string& token,
              const std::string& issuer, const std::string& audience) override;

 private:
  std::string _name;
};

class HealthHandler final : public v1::HealthIf {
 public:
  HealthHandler() = default;

  void check(std::map<std::string, std::string>& response) override;
};
}  // namespace loquat
