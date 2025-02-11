#include "marguerite/casbin.hpp"
#include "marguerite/minio.hpp"
#include "marguerite/sodium.hpp"
#include "marguerite/tink.hpp"
#include "marguerite/version.hpp"

#include <casbin/casbin.h>
#include <curl/curl.h>
#include <libpq-fe.h>
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

static std::shared_ptr<marguerite::Minio> gl_minio;
static std::shared_ptr<marguerite::Jwt> gl_jwt;
static std::shared_ptr<marguerite::Aes> gl_aes;
static std::shared_ptr<marguerite::HMac> gl_hmac;
static std::shared_ptr<casbin::Enforcer> gl_casbin_enforcer;

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

  std::set<std::string> audiences = {audience.value()};
  const auto now = absl::Now();
  const auto token = gl_jwt->sign(
      "", "", issuer.value(), subject.value(), audiences, now,
      absl::FromUnixSeconds(static_cast<int64_t>(not_before.value())),
      absl::FromUnixSeconds(static_cast<int64_t>(expires_at.value())), payload);

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

  try {
    const auto [_jwt_id, _key_id, subject, payload] =
        gl_jwt->verify(token.value(), issuer.value(), audience.value());

    auto subject_bin = marguerite::erlang::new_binary(env, subject);

    if (payload) {
      auto payload_bin = marguerite::erlang::new_binary(env, payload.value());
      return enif_make_tuple2(env, subject_bin, payload_bin);
    }

    return enif_make_tuple1(env, subject_bin);
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

  try {
    std::string buf(plain->begin(), plain->end());
    const auto code = gl_aes->encrypt(buf);
    return marguerite::erlang::new_binary(env, code);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}

static ERL_NIF_TERM aes_decrypt_nif(ErlNifEnv* env, int argc,
                                    const ERL_NIF_TERM argv[]) {
  const auto code = marguerite::erlang::get_binary(env, argv[0]);
  if (!code.has_value()) {
    return enif_make_badarg(env);
  }

  try {
    std::string buf(code->begin(), code->end());

    const auto plain = gl_aes->decrypt(buf);
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

  try {
    const auto code = gl_hmac->sign(plain.value());
    return marguerite::erlang::new_binary(env, code);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
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

  try {
    gl_hmac->verify(code.value(), plain.value());
    return enif_make_atom(env, "true");
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}

// ----------------------------------------------------------------------------

static ERL_NIF_TERM s3_create_bucket_nif(ErlNifEnv* env, int argc,
                                         const ERL_NIF_TERM argv[]) {
  const auto name = marguerite::erlang::get_binary(env, argv[0]);
  if (!name.has_value()) {
    return enif_make_badarg(env);
  }
  const auto is_public = marguerite::erlang::get_uint(env, argv[1]);
  const auto expiration_days = marguerite::erlang::get_uint(env, argv[2]);

  try {
    if (!gl_minio->bucket_exist(name.value())) {
      gl_minio->create_bucket(
          name.value(), is_public.value_or(0) == 1,
          expiration_days
              ? std::optional<std::chrono::days>{expiration_days.value()}
              : std::nullopt);
    }
    return enif_make_atom(env, "true");
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}

static ERL_NIF_TERM s3_get_presigned_object_url_nif(ErlNifEnv* env, int argc,
                                                    const ERL_NIF_TERM argv[]) {
  const auto bucket = marguerite::erlang::get_binary(env, argv[0]);
  if (!bucket.has_value()) {
    return enif_make_badarg(env);
  }
  const auto object = marguerite::erlang::get_binary(env, argv[1]);
  if (!object.has_value()) {
    return enif_make_badarg(env);
  }
  const auto expiry_seconds = marguerite::erlang::get_uint(env, argv[2]);
  try {
    const auto url = gl_minio->get_presigned_object_url(
        bucket.value(), object.value(),
        expiry_seconds ? std::chrono::seconds{expiry_seconds.value()}
                       : std::chrono::duration_cast<std::chrono::seconds>(
                             std::chrono::days{7}));
    return marguerite::erlang::new_binary(env, url);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}
static ERL_NIF_TERM s3_get_permanent_object_url_nif(ErlNifEnv* env, int argc,
                                                    const ERL_NIF_TERM argv[]) {
  const auto bucket = marguerite::erlang::get_binary(env, argv[0]);
  if (!bucket.has_value()) {
    return enif_make_badarg(env);
  }
  const auto object = marguerite::erlang::get_binary(env, argv[1]);
  if (!object.has_value()) {
    return enif_make_badarg(env);
  }
  try {
    const auto url =
        gl_minio->get_permanent_object_url(bucket.value(), object.value());
    return marguerite::erlang::new_binary(env, url);
  } catch (...) {
  }
  return enif_make_atom(env, "false");
}
static ERL_NIF_TERM s3_put_object_nif(ErlNifEnv* env, int argc,
                                      const ERL_NIF_TERM argv[]) {
  const auto bucket = marguerite::erlang::get_binary(env, argv[0]);
  if (!bucket.has_value()) {
    return enif_make_badarg(env);
  }
  const auto file = marguerite::erlang::get_binary(env, argv[1]);
  if (!file.has_value()) {
    return enif_make_badarg(env);
  }
  try {
    const auto [object, size] =
        gl_minio->put_object(bucket.value(), file.value());
    auto object_bin = marguerite::erlang::new_binary(env, object);
    return enif_make_tuple2(env, object_bin, enif_make_uint64(env, size));
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
  // spdlog::debug("OpenSSL v{}", OPENSSL_VERSION_STR);

  {
    if (sodium_init() < 0) {
      spdlog::error("failed to init sodium library");
    }
    spdlog::debug("sodium {}", SODIUM_VERSION_STRING);
  }

  {
    const auto v = PQlibVersion();
    spdlog::debug("libpq v{}.{}.{}", v / (100 * 100), (v / 100) % 100,
                  v % (100 * 100));
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

  const auto config = toml::parse_file("marguerite.toml");
  {
    auto node = config["minio"].as_table();
    gl_minio = std::make_shared<marguerite::Minio>(*node);
  }
  {
    const auto model = casbin::Model::NewModelFromString(R"INI(
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
)INI");

    auto node = config["postgresql"].as_table();
    std::shared_ptr<marguerite::casbin::PostgreSqlAdapter> adapter =
        std::make_shared<marguerite::casbin::PostgreSqlAdapter>(*node);

    gl_casbin_enforcer = std::make_shared<casbin::Enforcer>(
        model, std::dynamic_pointer_cast<casbin::Adapter>(adapter));
  }
  gl_hmac = std::make_shared<marguerite::HMac>();
  gl_aes = std::make_shared<marguerite::Aes>();
  gl_jwt = std::make_shared<marguerite::Jwt>();

  return EXIT_SUCCESS;
}

// ----------------------------------------------------------------------------

static ErlNifFunc nif_funcs[] = {
    {"version", 0, version_nif},
    {"s3_create_bucket", 1, s3_create_bucket_nif},
    {"s3_create_bucket", 2, s3_create_bucket_nif},
    {"s3_create_bucket", 3, s3_create_bucket_nif},
    {"s3_put_object", 2, s3_put_object_nif},
    {"s3_get_presigned_object_url", 2, s3_get_presigned_object_url_nif},
    {"s3_get_presigned_object_url", 3, s3_get_presigned_object_url_nif},
    {"s3_get_permanent_object_url", 2, s3_get_permanent_object_url_nif},
    {"aes_encrypt", 1, aes_encrypt_nif},
    {"aes_decrypt", 1, aes_decrypt_nif},
    {"hmac_sign", 1, hmac_sign_nif},
    {"hmac_verify", 2, hmac_verify_nif},
    {"jwt_sign", 5, jwt_sign_nif},
    {"jwt_sign", 6, jwt_sign_nif},
    {"jwt_verify", 3, jwt_verify_nif}};

// marguerite
ERL_NIF_INIT(Elixir.Marguerite.NIF, nif_funcs, load, NULL, NULL, NULL)
