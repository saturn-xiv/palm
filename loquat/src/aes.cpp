#include "loquat/env.hpp"

#include <tink/aead.h>
#include <tink/aead/aead_config.h>
#include <tink/aead/aes_gcm_key_manager.h>

std::optional<std::string> loquat::Aes::encrypt(
    const std::string& plain, const std::string& associated_data) {
  auto aes = this->load();
  auto cipher_r = aes->Encrypt(plain, associated_data);
  {
    const auto status = cipher_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto code = std::move(cipher_r.value());
  return code;
}

std::optional<std::string> loquat::Aes::decrypt(
    const std::string& cipher, const std::string& associated_data) {
  auto aes = this->load();
  auto plain_r = aes->Decrypt(cipher, associated_data);
  {
    const auto status = plain_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto plain = std::move(plain_r.value());
  return plain;
}

std::unique_ptr<crypto::tink::Aead> loquat::Aes::load() {
  auto keyset = this->Keyset::load(crypto::tink::AeadKeyTemplates::Aes256Gcm());
  auto aes_r = keyset->GetPrimitive<crypto::tink::Aead>(
      crypto::tink::ConfigGlobalRegistry());
  {
    const auto status = aes_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return nullptr;
    }
  }
  auto aes = std::move(aes_r.value());
  return aes;
}
