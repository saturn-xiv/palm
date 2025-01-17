#include "marguerite/env.hpp"
#include "marguerite/version.hpp"

#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>

#include <erl_nif.h>

static ERL_NIF_TERM version_nif(ErlNifEnv* env, int argc,
                                const ERL_NIF_TERM argv[]) {
  return enif_make_string(env, marguerite::GIT_VERSION.c_str(), ERL_NIF_UTF8);
}

static ERL_NIF_TERM hmac_sign_nif(ErlNifEnv* env, int argc,
                                  const ERL_NIF_TERM argv[]) {
  unsigned plain_len;
  if (!enif_get_string_length(env, argv[0], &plain_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }
  std::string plain(plain_len, ' ');
  if (!enif_get_string(env, argv[0], plain.data(), plain_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }

  marguerite::HMac hmac;
  const auto code = hmac.sign(plain);
  return enif_make_string(env, code.c_str(), ERL_NIF_UTF8);
}

static ERL_NIF_TERM hmac_verify_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  unsigned code_len;
  if (!enif_get_string_length(env, argv[0], &code_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }
  std::string code(code_len, ' ');
  if (!enif_get_string(env, argv[0], code.data(), code_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }

  unsigned plain_len;
  if (!enif_get_string_length(env, argv[1], &plain_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }
  std::string plain(plain_len, ' ');
  if (!enif_get_string(env, argv[1], plain.data(), plain_len, ERL_NIF_UTF8)) {
    return enif_make_badarg(env);
  }

  marguerite::HMac hmac;
  try {
    hmac.verify(code, plain);
    return enif_make_atom(env, "true");
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}

static int load(ErlNifEnv* env, void** priv_data, ERL_NIF_TERM load_info) {
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

  return EXIT_SUCCESS;
}

static ErlNifFunc nif_funcs[] = {{"version", 0, version_nif},
                                 {"hmac_sign", 1, hmac_sign_nif},
                                 {"hmac_verify", 2, hmac_verify_nif}};

ERL_NIF_INIT(marguerite, nif_funcs, load, NULL, NULL, NULL)
