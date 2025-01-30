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

static inline std::optional<std::string> get_binary(ErlNifEnv* env,
                                                    ERL_NIF_TERM buf) {
  ErlNifBinary bin;
  if (!enif_inspect_binary(env, buf, &bin)) {
    return std::nullopt;
  }
  std::string it((char*)bin.data, bin.size);
  return {it};
}
}  // namespace erlang
}  // namespace marguerite

// ----------------------------------------------------------------------------

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

  ErlNifBinary bin;
  enif_alloc_binary(code.size(), &bin);
  std::strcpy((char*)bin.data, code.c_str());
  bin.size = code.size();
  return enif_make_binary(env, &bin);
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

    ErlNifBinary bin;
    enif_alloc_binary(plain.size(), &bin);
    std::strcpy((char*)bin.data, plain.c_str());
    bin.size = plain.size();
    return enif_make_binary(env, &bin);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}
// ----------------------------------------------------------------------------

static ERL_NIF_TERM hmac_sign_nif(ErlNifEnv* env, int argc,
                                  const ERL_NIF_TERM argv[]) {
  unsigned plain_len;
  if (!enif_get_string_length(env, argv[0], &plain_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }
  const auto plain = marguerite::erlang::get_string(env, argv[0]);
  if (!plain.has_value()) {
    return enif_make_badarg(env);
  }

  marguerite::HMac hmac;
  const auto code = hmac.sign(plain.value());

  ErlNifBinary bin;
  enif_alloc_binary(code.size(), &bin);
  std::strcpy((char*)bin.data, code.c_str());
  bin.size = code.size();
  return enif_make_binary(env, &bin);
}

static ERL_NIF_TERM hmac_verify_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  const auto code = marguerite::erlang::get_binary(env, argv[0]);
  if (!code.has_value()) {
    return enif_make_badarg(env);
  }

  const auto plain = marguerite::erlang::get_string(env, argv[1]);
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

static ErlNifFunc nif_funcs[] = {{"version", 0, version_nif},
                                 {"aes_encrypt", 1, aes_encrypt_nif},
                                 {"aes_decrypt", 1, aes_decrypt_nif},
                                 {"hmac_sign", 1, hmac_sign_nif},
                                 {"hmac_verify", 2, hmac_verify_nif}};

ERL_NIF_INIT(marguerite, nif_funcs, load, NULL, NULL, NULL)
