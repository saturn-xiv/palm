#include "loquat/env.hpp"
#include "loquat/version.hpp"

#include <tink/aead.h>
#include <tink/aead/aead_config.h>
#include <tink/aead/aes_gcm_key_manager.h>
#include <tink/binary_keyset_reader.h>
#include <tink/binary_keyset_writer.h>
#include <tink/cleartext_keyset_handle.h>
#include <tink/jwt/internal/jwt_mac_impl.h>
#include <tink/jwt/internal/jwt_mac_internal.h>
#include <tink/jwt/jwt_key_templates.h>
#include <tink/jwt/jwt_signature_config.h>
#include <tink/jwt/jwt_validator.h>
#include <tink/jwt/raw_jwt.h>
#include <tink/mac/mac_factory.h>
#include <tink/public_key_sign.h>
#include <tink/public_key_verify.h>
#include <tink/signature/signature_key_templates.h>
#include <tink/tink_config.h>
#include <tink/util/status.h>

std::string loquat::Jwt::sign(
    const std::optional<std::string> jwt_id,
    const std::optional<std::string> key_id, const std::string& issuer,
    const std::string& subject, const std::set<std::string> audiences,
    const absl::Time& issued_at, const absl::Time& not_before,
    const absl::Time& expired_at, const std::optional<std::string> payload) {
  spdlog::debug(
      "sign token for jwt-id({}) key-id({}) issuer({}) subject({}) "
      "audiences({})",
      jwt_id.value_or(""), key_id.value_or(""), issuer, subject,
      absl::StrJoin(audiences, ","));
  // https://github.com/tink-crypto/tink-cc/blob/main/tink/jwt/raw_jwt.h#L101
  auto raw_rb = crypto::tink::RawJwtBuilder()
                    .SetIssuer(issuer)
                    .SetSubject(subject)
                    .SetNotBefore(not_before)
                    .SetIssuedAt(issued_at)
                    .SetExpiration(expired_at);
  if (jwt_id) {
    raw_rb = raw_rb.SetJwtId(jwt_id.value());
  }
  for (const auto& it : audiences) {
    raw_rb = raw_rb.AddAudience(it);
  }
  if (payload) {
    raw_rb =
        raw_rb.AddJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY, payload.value());
  }

  auto raw_r = raw_rb.Build();
  this->check(raw_r);
  auto raw = std::move(raw_r.value());
  auto jwt = this->load();
  auto token_r = jwt->ComputeMacAndEncode(raw);
  this->check(token_r);
  auto token = std::move(token_r.value());
  return token;
}

std::tuple<std::optional<std::string>, std::optional<std::string>, std::string,
           std::optional<std::string>>
loquat::Jwt::verify(const std::string& token, const std::string& issuer,
                    const std::string& audience) {
  spdlog::debug("verify issuer({}) audience({}) token({})", issuer, audience,
                token);
  auto validator_b = crypto::tink::JwtValidatorBuilder()
                         .IgnoreTypeHeader()
                         .ExpectIssuer(issuer)
                         .ExpectAudience(audience)
                         .ExpectIssuedInThePast();

  auto validator_r = validator_b.Build();
  this->check(validator_r);
  auto validator = std::move(validator_r.value());

  auto jwt = this->load();
  auto payload_r = jwt->VerifyMacAndDecode(token, validator);
  this->check(payload_r);
  auto payload = std::move(payload_r.value());

  // https://github.com/tink-crypto/tink-cc/blob/main/tink/jwt/verified_jwt.h#L53
  auto subject_r = payload.GetSubject();
  this->check(subject_r);
  auto subject = std::move(subject_r.value());

  std::optional<std::string> jwt_id = std::nullopt;
  if (payload.HasJwtId()) {
    auto ir = payload.GetJwtId();
    this->check(ir);
    auto iv = std::move(ir.value());
    jwt_id = std::optional<std::string>{iv};
  }
  std::optional<std::string> key_id = std::nullopt;
  std::optional<std::string> payload_ = std::nullopt;
  if (payload.HasJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY)) {
    auto ir = payload.GetJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY);
    this->check(ir);
    auto iv = std::move(ir.value());
    payload_ = std::optional<std::string>{iv};
  }

  spdlog::debug("get jwt-id({}) key-id({}) subject({})", jwt_id.value_or(""),
                key_id.value_or(""), subject);
  std::tuple<std::optional<std::string>, std::optional<std::string>,
             std::string, std::optional<std::string>>
      it = std::make_tuple(jwt_id, key_id, subject, payload_);
  return it;
}

std::unique_ptr<crypto::tink::JwtMac> loquat::Jwt::load() {
  auto keyset = this->Keyset::load(crypto::tink::JwtHs512Template());
  auto jwt_r = keyset->GetPrimitive<crypto::tink::JwtMac>(
      crypto::tink::ConfigGlobalRegistry());
  this->check(jwt_r);
  auto jwt = std::move(jwt_r.value());
  return jwt;
}

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

std::string loquat::Aes::encrypt(const std::string& plain) {
  auto aes = this->load();
  auto code_r = aes->Encrypt(plain, "");
  this->check(code_r);
  auto code = std::move(code_r.value());
  return code;
}

std::string loquat::Aes::decrypt(const std::string& code) {
  auto aes = this->load();
  auto plain_r = aes->Decrypt(code, "");
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

std::unique_ptr<crypto::tink::KeysetHandle> loquat::Keyset::load(
    const google::crypto::tink::KeyTemplate& tpl) {
  const std::lock_guard<std::mutex> lock(this->_locker);

  const auto file = this->keyset();
  if (std::filesystem::exists(file)) {
    spdlog::debug("load keyset from {}", file.string());

    if (std::filesystem::status(file).permissions() !=
        std::filesystem::perms::owner_read) {
      throw std::invalid_argument("key file too open");
    }

    std::unique_ptr<std::ifstream> in =
        std::make_unique<std::ifstream>(file, std::ios_base::binary);
    auto reader_r = crypto::tink::BinaryKeysetReader::New(std::move(in));
    this->check(reader_r);
    auto reader = std::move(reader_r.value());
    auto keyset_handle_r =
        crypto::tink::CleartextKeysetHandle::Read(std::move(reader));
    this->check(keyset_handle_r);
    auto keyset_handle = std::move(keyset_handle_r.value());
    return keyset_handle;

  } else {
    spdlog::warn("not exists, try to create {}", file.string());

    auto keyset_handle_r = crypto::tink::KeysetHandle::GenerateNew(
        tpl, crypto::tink::KeyGenConfigGlobalRegistry());
    this->check(keyset_handle_r);
    auto keyset_handler = std::move(keyset_handle_r.value());
    {
      std::unique_ptr<std::ofstream> out = std::make_unique<std::ofstream>();
      out->open(file, std::ios_base::binary);
      auto writer_r = crypto::tink::BinaryKeysetWriter::New(std::move(out));
      this->check(writer_r);
      auto writer = std::move(writer_r.value());
      const auto status = crypto::tink::CleartextKeysetHandle::Write(
          writer.get(), *keyset_handler.get());
      this->check(status);
    }
    std::filesystem::permissions(file, std::filesystem::perms::owner_read);
    return keyset_handler;
  }
}
