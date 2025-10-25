#include "marigold/client.hpp"
#include "marigold/version.hpp"

#include <memory>

#include <erl_nif.h>
#include <openssl/opensslv.h>

static std::shared_ptr<marigold::S3> gl_aes;

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

static ERL_NIF_TERM version_nif(ErlNifEnv* env, int argc,
                                const ERL_NIF_TERM argv[]) {
  if (argc != 0) {
    return enif_make_badarg(env);
  }
  return enif_make_string(env, marigold::VERSION.c_str(), ERL_NIF_LATIN1);
}

static int on_load(ErlNifEnv* env, void** priv_data, ERL_NIF_TERM load_info) {
  spdlog::set_level(spdlog::level::debug);

  spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);

  return EXIT_SUCCESS;
}

static ErlNifFunc nif_funcs[] = {
    {"version", 0, version_nif}};

ERL_NIF_INIT(marigold, nif_funcs, on_load, NULL, NULL, NULL)
