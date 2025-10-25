#include "tulip/tink.hpp"
#include "tulip/version.hpp"

#include <cstring>
#include <memory>

#include <erl_nif.h>
#include <openssl/opensslv.h>
#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>

static std::shared_ptr<tulip::Aes> gl_aes;
static std::shared_ptr<tulip::HMac> gl_hmac;
static std::shared_ptr<tulip::Jwt> gl_jwt;

static inline std::optional<std::set<std::string>> get_string_array_from_env(
    ErlNifEnv* env, ERL_NIF_TERM item) {
  if (!enif_is_list(env, item)) {
    return std::nullopt;
  }
  std::set<std::string> items;
  unsigned len;
  if (!enif_get_list_length(env, item, &len)) {
    return std::nullopt;
  }
  //   TODO
  return items;
}

static inline std::optional<std::string> get_string_from_env(
    ErlNifEnv* env, ERL_NIF_TERM item) {
  ErlNifBinary bin;

  if (!enif_is_binary(env, item)) {
    return std::nullopt;
  }
  if (!enif_inspect_binary(env, item, &bin)) {
    return std::nullopt;
  }

  std::string str((const char*)bin.data, bin.size);
  return str;
}

static inline bool copy_string_to_binary(const std::string str,
                                         ErlNifBinary* bin) {
  if (!enif_alloc_binary(str.size(), bin)) {
    return false;
  }
  std::memcpy(bin->data, str.data(), str.size());
  return true;
}

static ERL_NIF_TERM aes_encrypt_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 1) {
    return enif_make_badarg(env);
  }

  auto plain = get_string_from_env(env, argv[0]);
  if (!plain) {
    return enif_make_badarg(env);
  }
  spdlog::debug("encrypt {}", plain.value());
  try {
    const auto code = gl_aes->encrypt(plain.value());

    ErlNifBinary code_b;
    if (!copy_string_to_binary(code, &code_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_binary(env, &code_b);
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM aes_decrypt_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 1) {
    return enif_make_badarg(env);
  }

  auto code = get_string_from_env(env, argv[0]);
  if (!code) {
    return enif_make_badarg(env);
  }

  try {
    const auto plain = gl_aes->decrypt(code.value());

    ErlNifBinary plain_b;
    if (!copy_string_to_binary(plain, &plain_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_binary(env, &plain_b);
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM hmac_sign_nif(ErlNifEnv* env, int argc,
                                  const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 1) {
    return enif_make_badarg(env);
  }

  auto plain = get_string_from_env(env, argv[0]);
  if (!plain) {
    return enif_make_badarg(env);
  }

  try {
    const auto code = gl_hmac->sign(plain.value());

    ErlNifBinary code_b;
    if (!copy_string_to_binary(code, &code_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_binary(env, &code_b);
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM hmac_verify_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 2) {
    return enif_make_badarg(env);
  }

  auto code = get_string_from_env(env, argv[0]);
  if (!code) {
    return enif_make_badarg(env);
  }
  auto plain = get_string_from_env(env, argv[1]);
  if (!plain) {
    return enif_make_badarg(env);
  }

  try {
    gl_hmac->verify(code.value(), plain.value());
    return enif_make_atom(env, "true");
  } catch (const std::runtime_error& err) {
    return enif_make_atom(env, "false");
  }
}

static ERL_NIF_TERM jwt_sign_nif_4(ErlNifEnv* env, int argc,
                                   const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 4) {
    return enif_make_badarg(env);
  }

  auto issuer = get_string_from_env(env, argv[0]);
  if (!issuer) {
    return enif_make_badarg(env);
  }
  auto subject = get_string_from_env(env, argv[1]);
  if (!subject) {
    return enif_make_badarg(env);
  }

  auto audiences = get_string_array_from_env(env, argv[2]);
  if (!audiences) {
    return enif_make_badarg(env);
  }
  int ttl_in_seconds;
  if (!enif_get_int(env, argv[3], &ttl_in_seconds)) {
    return enif_make_badarg(env);
  }
  auto now = absl::Now();
  auto exp = now + absl::Seconds(ttl_in_seconds);

  try {
    const auto token = gl_jwt->sign(std::nullopt, std::nullopt, issuer.value(),
                                    subject.value(), audiences.value(), now,
                                    now, exp, std::nullopt);

    ErlNifBinary token_b;
    if (!copy_string_to_binary(token, &token_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_binary(env, &token_b);
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM jwt_sign_nif_3(ErlNifEnv* env, int argc,
                                   const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 3) {
    return enif_make_badarg(env);
  }

  auto issuer = get_string_from_env(env, argv[0]);
  if (!issuer) {
    return enif_make_badarg(env);
  }
  auto subject = get_string_from_env(env, argv[1]);
  if (!subject) {
    return enif_make_badarg(env);
  }

  int ttl_in_seconds;
  if (!enif_get_int(env, argv[2], &ttl_in_seconds)) {
    return enif_make_badarg(env);
  }
  auto now = absl::Now();
  auto exp = now + absl::Seconds(ttl_in_seconds);

  try {
    const auto token =
        gl_jwt->sign(std::nullopt, std::nullopt, issuer.value(),
                     subject.value(), {}, now, now, exp, std::nullopt);

    ErlNifBinary token_b;
    if (!copy_string_to_binary(token, &token_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_binary(env, &token_b);
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM jwt_verify_nif_3(ErlNifEnv* env, int argc,
                                     const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 3) {
    return enif_make_badarg(env);
  }

  auto token = get_string_from_env(env, argv[0]);
  if (!token) {
    return enif_make_badarg(env);
  }
  auto issuer = get_string_from_env(env, argv[1]);
  if (!issuer) {
    return enif_make_badarg(env);
  }
  auto audience = get_string_from_env(env, argv[2]);
  if (!audience) {
    return enif_make_badarg(env);
  }

  try {
    const auto [jwt_id, key_id, subject, payload] =
        gl_jwt->verify(token.value(), issuer.value(), audience.value());

    ErlNifBinary subject_b;
    if (!copy_string_to_binary(subject, &subject_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_tuple1(env, enif_make_binary(env, &subject_b));
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}

static ERL_NIF_TERM jwt_verify_nif_2(ErlNifEnv* env, int argc,
                                     const ERL_NIF_TERM argv[]) {
  spdlog::debug("call {}", __PRETTY_FUNCTION__);
  if (argc != 2) {
    return enif_make_badarg(env);
  }

  auto token = get_string_from_env(env, argv[0]);
  if (!token) {
    return enif_make_badarg(env);
  }
  auto issuer = get_string_from_env(env, argv[1]);
  if (!issuer) {
    return enif_make_badarg(env);
  }
  try {
    const auto [jwt_id, key_id, subject, payload] =
        gl_jwt->verify(token.value(), issuer.value());

    ErlNifBinary subject_b;
    if (!copy_string_to_binary(subject, &subject_b)) {
      return enif_make_badarg(env);
    }

    return enif_make_tuple1(env, enif_make_binary(env, &subject_b));
  } catch (const std::runtime_error& err) {
    return enif_make_badarg(env);
  }
}
static ERL_NIF_TERM version_nif(ErlNifEnv* env, int argc,
                                const ERL_NIF_TERM argv[]) {
  if (argc != 0) {
    return enif_make_badarg(env);
  }
  return enif_make_string(env, tulip::VERSION.c_str(), ERL_NIF_LATIN1);
}

static int on_load(ErlNifEnv* env, void** priv_data, ERL_NIF_TERM load_info) {
  spdlog::set_level(spdlog::level::debug);

  spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);
  spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
  spdlog::debug(
      "Protocol Buffers v{}",
      google::protobuf::internal::VersionString(GOOGLE_PROTOBUF_VERSION));

  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      spdlog::error("failed to register tink");
      return EXIT_FAILURE;
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      spdlog::error("failed to register tink-jwt");
      return EXIT_FAILURE;
    }
  }

  {
    gl_aes = std::make_shared<tulip::Aes>(tulip::PROJECT_NAME + ".aes");
    gl_hmac = std::make_shared<tulip::HMac>(tulip::PROJECT_NAME + ".hmac");
    gl_jwt = std::make_shared<tulip::Jwt>(tulip::PROJECT_NAME + ".jwt");
  }

  return EXIT_SUCCESS;
}

static ErlNifFunc nif_funcs[] = {
    {"aes_encrypt", 1, aes_encrypt_nif}, {"aes_decrypt", 1, aes_decrypt_nif},
    {"hmac_sign", 1, hmac_sign_nif},     {"hmac_verify", 2, hmac_verify_nif},
    {"jwt_sign", 3, jwt_sign_nif_3},     {"jwt_sign", 4, jwt_sign_nif_4},
    {"jwt_verify", 2, jwt_verify_nif_2}, {"jwt_verify", 3, jwt_verify_nif_3},
    {"version", 0, version_nif}};

ERL_NIF_INIT(tulip, nif_funcs, on_load, NULL, NULL, NULL)
