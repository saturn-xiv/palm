#include "loquat/env.hpp"

std::string loquat::HMac::sign(const std::string& plain) {
  auto mac = this->load();
  auto code_r = mac->ComputeMac(plain);
  this->check(code_r);
  auto code = std::move(code_r.value());
  return code;
}

void loquat::HMac::verify(const std::string& code, const std::string& plain) {
  auto mac = this->load();
  auto status = mac->VerifyMac(code, plain);
  this->check(status);
}

std::unique_ptr<crypto::tink::Mac> loquat::HMac::load() {
  auto keyset = this->Keyset::load(crypto::tink::MacKeyTemplates::HmacSha512());
  auto mac_r = keyset->GetPrimitive<crypto::tink::Mac>(
      crypto::tink::ConfigGlobalRegistry());
  this->check(mac_r);
  auto mac = std::move(mac_r.value());
  return mac;
}
