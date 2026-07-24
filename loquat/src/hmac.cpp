#include "loquat/env.hpp"

std::optional<std::string> loquat::HMac::sign(const std::string& plain) {
  auto mac = this->load();
  auto code_r = mac->ComputeMac(plain);
  {
    const auto status = code_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto code = std::move(code_r.value());
  return code;
}

bool loquat::HMac::verify(const std::string& code, const std::string& plain) {
  auto mac = this->load();
  auto status = mac->VerifyMac(code, plain);
  if (status.ok()) {
    return true;
  }
  spdlog::error("{}", status.message());
  return false;
}

std::unique_ptr<crypto::tink::Mac> loquat::HMac::load() {
  auto keyset = this->Keyset::load(crypto::tink::MacKeyTemplates::HmacSha512());
  auto mac_r = keyset->GetPrimitive<crypto::tink::Mac>(
      crypto::tink::ConfigGlobalRegistry());
  {
    const auto status = mac_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return nullptr;
    }
  }
  auto mac = std::move(mac_r.value());
  return mac;
}
