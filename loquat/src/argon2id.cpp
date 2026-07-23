#include "loquat/env.hpp"

#include <argon2.h>

// Argon2id parameters recommended by RFC 9106
// Output length in bytes
#define LOQUAT_ARGON2ID_HASH_LEN 32
// Iterations (Time)
#define LOQUAT_ARGON2ID_T_COST 3
// Memory in KiB (64 MiB)
#define LOQUAT_ARGON2ID_M_COST 65536
// Threads
#define LOQUAT_ARGON2ID_PARALLELISM 1

static inline std::vector<uint8_t> random_bytes(size_t salt_length) {
  std::random_device rd;
  std::mt19937_64 engine(rd());
  std::uniform_int_distribution<std::uint16_t> dist(0, 255);
  std::vector<std::uint8_t> buf(salt_length);
  std::ranges::generate(
      buf, [&]() { return static_cast<std::uint8_t>(dist(engine)); });
  return buf;
}

static inline std::vector<uint8_t> sign_argon2id(
    const std::string& password, const std::vector<uint8_t>& salt) {
  if (salt.size() < 8) {
    throw std::runtime_error("salt must be at least 8 bytes");
  }
  std::vector<uint8_t> hash(LOQUAT_ARGON2ID_HASH_LEN);
  {
    const int rc = argon2id_hash_raw(
        LOQUAT_ARGON2ID_T_COST, LOQUAT_ARGON2ID_M_COST,
        LOQUAT_ARGON2ID_PARALLELISM, password.data(), password.size(),
        salt.data(), salt.size(), hash.data(), hash.size());
    if (rc != ARGON2_OK) {
      const std::string err = std::format("{}", argon2_error_message(rc));
      throw std::runtime_error(err);
    }
  }
  return hash;
}

// https://github.com/p-h-c/phc-winner-argon2
std::pair<std::vector<uint8_t>, std::vector<uint8_t>> loquat::argon2id::sign(
    const std::string& password, size_t salt_length) {
  const auto salt = random_bytes(salt_length);
  const auto buf = sign_argon2id(password, salt);
  return {buf, salt};
}

bool loquat::argon2id::verify(const std::vector<uint8_t>& code,
                              const std::string& password,
                              const std::vector<uint8_t>& salt) {
  const auto buf = sign_argon2id(password, salt);
  return std::ranges::equal(std::as_bytes(std::span{buf}),
                            std::as_bytes(std::span{code}));
}
