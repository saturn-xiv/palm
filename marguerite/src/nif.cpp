#include "marguerite/env.hpp"
#include "marguerite/version.hpp"

#include <curl/curl.h>
#include <sodium.h>
#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>

#include <erl_nif.h>

namespace marguerite {
namespace erlang {
static inline std::optional<std::string> get_string(ErlNifEnv* env,
                                                    ERL_NIF_TERM buf) {
  unsigned len;
  if (!enif_get_string_length(env, buf, &len, ERL_NIF_UTF8)) {
    return std::nullopt;
  }
  std::string it(len, ' ');
  if (!enif_get_string(env, buf, it.data(), len, ERL_NIF_UTF8)) {
    return std::nullopt;
  }
  return {it};
}
static inline std::optional<unsigned int> get_uint(ErlNifEnv* env,
                                                   ERL_NIF_TERM buf) {
  unsigned int it;
  if (!enif_get_uint(env, buf, &it)) {
    return std::nullopt;
  }
  return {it};
}
static inline std::optional<std::string> get_binary(ErlNifEnv* env,
                                                    ERL_NIF_TERM buf) {
  ErlNifBinary bin;
  if (!enif_inspect_binary(env, buf, &bin)) {
    return std::nullopt;
  }
  std::string it((char*)bin.data, bin.size);
  return {it};
}
static inline ERL_NIF_TERM new_binary(ErlNifEnv* env, const std::string& s) {
  ErlNifBinary bin;
  enif_alloc_binary(s.size(), &bin);
  std::strcpy((char*)bin.data, s.c_str());
  bin.size = s.size();
  return enif_make_binary(env, &bin);
}
}  // namespace erlang
}  // namespace marguerite

// ----------------------------------------------------------------------------

static ERL_NIF_TERM jwt_sign_nif(ErlNifEnv* env, int argc,
                                 const ERL_NIF_TERM argv[]) {
  const auto issuer = marguerite::erlang::get_binary(env, argv[0]);
  if (!issuer.has_value()) {
    return enif_make_badarg(env);
  }
  const auto subject = marguerite::erlang::get_binary(env, argv[1]);
  if (!subject.has_value()) {
    return enif_make_badarg(env);
  }
  const auto audience = marguerite::erlang::get_binary(env, argv[2]);
  if (!audience.has_value()) {
    return enif_make_badarg(env);
  }
  const auto not_before = marguerite::erlang::get_uint(env, argv[3]);
  if (!not_before.has_value()) {
    return enif_make_badarg(env);
  }
  const auto expires_at = marguerite::erlang::get_uint(env, argv[4]);
  if (!expires_at.has_value()) {
    return enif_make_badarg(env);
  }
  const auto payload = marguerite::erlang::get_binary(env, argv[5]);

  marguerite::Jwt jwt;
  std::set<std::string> audiences = {audience.value()};
  const auto now = absl::Now();
  const auto token =
      jwt.sign("", "", issuer.value(), subject.value(), audiences, now,
               absl::FromUnixSeconds(not_before.value()),
               absl::FromUnixSeconds(expires_at.value()), payload);

  return marguerite::erlang::new_binary(env, token);
}

static ERL_NIF_TERM jwt_verify_nif(ErlNifEnv* env, int argc,
                                   const ERL_NIF_TERM argv[]) {
  const auto token = marguerite::erlang::get_binary(env, argv[0]);
  if (!token.has_value()) {
    return enif_make_badarg(env);
  }
  const auto issuer = marguerite::erlang::get_binary(env, argv[1]);
  if (!issuer.has_value()) {
    return enif_make_badarg(env);
  }
  const auto audience = marguerite::erlang::get_binary(env, argv[2]);
  if (!audience.has_value()) {
    return enif_make_badarg(env);
  }

  marguerite::Jwt jwt;
  try {
    const auto [_jwt_id, _key_id, subject, payload] =
        jwt.verify(token.value(), issuer.value(), audience.value());

    ErlNifBinary subject_bin;
    enif_alloc_binary(subject.size(), &subject_bin);
    std::strcpy((char*)subject_bin.data, subject.c_str());
    subject_bin.size = subject.size();

    if (payload) {
      ErlNifBinary payload_bin;
      enif_alloc_binary(payload->size(), &payload_bin);
      std::strcpy((char*)payload_bin.data, payload->c_str());
      payload_bin.size = payload->size();
      return enif_make_tuple2(env, enif_make_binary(env, &subject_bin),
                              enif_make_binary(env, &payload_bin));
    }

    return enif_make_tuple1(env, enif_make_binary(env, &subject_bin));
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}
// ----------------------------------------------------------------------------
static ERL_NIF_TERM aes_encrypt_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  const auto plain = marguerite::erlang::get_binary(env, argv[0]);
  if (!plain.has_value()) {
    return enif_make_badarg(env);
  }

  std::string buf(plain->begin(), plain->end());
  marguerite::Aes aes;
  const auto code = aes.encrypt(buf);

  return marguerite::erlang::new_binary(env, code);
}

static ERL_NIF_TERM aes_decrypt_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  const auto code = marguerite::erlang::get_binary(env, argv[0]);
  if (!code.has_value()) {
    return enif_make_badarg(env);
  }

  std::string buf(code->begin(), code->end());

  marguerite::Aes aes;
  try {
    const auto plain = aes.decrypt(buf);

    return marguerite::erlang::new_binary(env, plain);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}
// ----------------------------------------------------------------------------

static ERL_NIF_TERM hmac_sign_nif(ErlNifEnv* env, int argc,
                                  const ERL_NIF_TERM argv[]) {
  const auto plain = marguerite::erlang::get_binary(env, argv[0]);
  if (!plain.has_value()) {
    return enif_make_badarg(env);
  }

  marguerite::HMac hmac;
  const auto code = hmac.sign(plain.value());
  return marguerite::erlang::new_binary(env, code);
}

static ERL_NIF_TERM hmac_verify_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  const auto code = marguerite::erlang::get_binary(env, argv[0]);
  if (!code.has_value()) {
    return enif_make_badarg(env);
  }

  const auto plain = marguerite::erlang::get_binary(env, argv[1]);
  if (!plain.has_value()) {
    return enif_make_badarg(env);
  }

  marguerite::HMac hmac;
  try {
    hmac.verify(code.value(), plain.value());
    return enif_make_atom(env, "true");
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}

// ----------------------------------------------------------------------------

static ERL_NIF_TERM version_nif(ErlNifEnv* env, int argc,
                                const ERL_NIF_TERM argv[]) {
  return enif_make_string(env, marguerite::GIT_VERSION.c_str(), ERL_NIF_UTF8);
}

// ----------------------------------------------------------------------------

static int load(ErlNifEnv* env, void** priv_data, ERL_NIF_TERM load_info) {
  spdlog::set_level(spdlog::level::debug);
  spdlog::debug("curl {}", curl_version());
  spdlog::debug("ERL NIF {}", ERL_NIF_MIN_ERTS_VERSION);
  spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);

  {
    if (sodium_init() < 0) {
      spdlog::error("failed to init sodium library");
    }
    spdlog::debug("sodium {}", SODIUM_VERSION_STRING);
  }

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

  return EXIT_SUCCESS;
}

// ----------------------------------------------------------------------------

static ErlNifFunc nif_funcs[] = {
    {"version", 0, version_nif},         {"aes_encrypt", 1, aes_encrypt_nif},
    {"aes_decrypt", 1, aes_decrypt_nif}, {"hmac_sign", 1, hmac_sign_nif},
    {"hmac_verify", 2, hmac_verify_nif}, {"jwt_sign", 6, jwt_sign_nif},
    {"jwt_verify", 3, jwt_verify_nif}};

// marguerite
ERL_NIF_INIT(Elixir.Marguerite.NIF, nif_funcs, load, NULL, NULL, NULL)
