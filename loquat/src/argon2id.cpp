#include "loquat/env.hpp"

#include <sodium.h>

std::optional<std::string> loquat::argon2id::sign(const std::string& password) {
  char hashed[crypto_pwhash_STRBYTES];
  if (crypto_pwhash_str(hashed, password.data(), password.size(),
                        crypto_pwhash_OPSLIMIT_SENSITIVE,
                        crypto_pwhash_MEMLIMIT_SENSITIVE) != 0) {
    spdlog::error("out of memory");
    return std::nullopt;
  }
  return hashed;
}

bool loquat::argon2id::verify(const std::string& hashed,
                              const std::string& password) {
  return crypto_pwhash_str_verify(hashed.data(), password.data(),
                                  password.size()) == 0;
}
