#include "loquat/env.hpp"

#include <tink/aead.h>
#include <tink/aead/aead_config.h>
#include <tink/aead/aes_gcm_key_manager.h>

std::string loquat::Aes::encrypt(const std::string& plain,
                                 const std::string& associated_data) {
  auto aes = this->load();
  auto code_r = aes->Encrypt(plain, associated_data);
  this->check(code_r);
  auto code = std::move(code_r.value());
  return code;
}

std::string loquat::Aes::decrypt(const std::string& code,
                                 const std::string& associated_data) {
  auto aes = this->load();
  auto plain_r = aes->Decrypt(code, associated_data);
  this->check(plain_r);
  auto plain = std::move(plain_r.value());
  return plain;
}

std::unique_ptr<crypto::tink::Aead> loquat::Aes::load() {
  auto keyset = this->Keyset::load(crypto::tink::AeadKeyTemplates::Aes256Gcm());
  auto aes_r = keyset->GetPrimitive<crypto::tink::Aead>(
      crypto::tink::ConfigGlobalRegistry());
  this->check(aes_r);
  auto aes = std::move(aes_r.value());
  return aes;
}
