#pragma once

#include <chrono>
#include <climits>
#include <cstdlib>
#include <ctime>
#include <random>
#include <string>
#include <vector>

namespace palm {

class Aes {
 public:
  Aes(/* A 256 bit key */ const std::string& key,
      /* A 128 bit IV */ const std::string& iv);
  std::vector<uint8_t> encrypt(const std::vector<uint8_t> plain) const;
  inline std::vector<uint8_t> encrypt(const std::string& plain) const {
    std::vector<uint8_t> it(plain.begin(), plain.end());
    return encrypt(it);
  }
  std::vector<uint8_t> decrypt(const std::vector<uint8_t> code) const;

 private:
  std::vector<uint8_t> _key;
  std::vector<uint8_t> _iv;
  static const size_t BUFFER_MAX_SIZE = 1 << 12;
};

class HMac {
 public:
  HMac(const std::string& key);
  std::vector<uint8_t> sign(const std::vector<uint8_t> plain) const;
  inline std::vector<uint8_t> sign(const std::string& plain) const {
    std::vector<uint8_t> it(plain.begin(), plain.end());
    return sign(it);
  }
  bool verify(const std::vector<uint8_t> code,
              const std::vector<uint8_t> plain) const {
    return sign(plain) == code;
  }
  inline bool verify(const std::vector<uint8_t> code,
                     const std::string& plain) const {
    std::vector<uint8_t> it(plain.begin(), plain.end());
    return verify(code, it);
  }

 private:
  std::vector<uint8_t> _key;
};
namespace sha512 {
std::vector<uint8_t> sign(const std::vector<uint8_t> plain);

inline std::vector<uint8_t> sign(const std::string& plain) {
  std::vector<uint8_t> it(plain.begin(), plain.end());
  return sign(it);
}
inline bool verify(const std::vector<uint8_t> code,
                   const std::vector<uint8_t> plain) {
  return sign(plain) == code;
}
inline bool verify(const std::vector<uint8_t> code, const std::string& plain) {
  return sign(plain) == code;
}
}  // namespace sha512

namespace sha256 {
std::vector<uint8_t> sign(const std::vector<uint8_t> plain);

inline std::vector<uint8_t> sign(const std::string& plain) {
  std::vector<uint8_t> it(plain.begin(), plain.end());
  return sign(it);
}
inline bool verify(const std::vector<uint8_t> code,
                   const std::vector<uint8_t> plain) {
  return sign(plain) == code;
}
inline bool verify(const std::vector<uint8_t> code, const std::string& plain) {
  return sign(plain) == code;
}
}  // namespace sha256

namespace base64 {
std::string to_string(const std::vector<uint8_t> buf);
std::vector<uint8_t> from_string(const std::string& str);
}  // namespace base64

namespace random {
std::vector<uint8_t> bytes(size_t len);
std::string alphanumeric(size_t len);
inline double double_(double min = 0.0, double max = 1.0) {
  static std::mt19937 rng(std::time(nullptr));
  std::uniform_real_distribution<double> dist(min, max);
  return dist(rng);
}
inline std::tuple<uint8_t, uint8_t, uint8_t> rgb() {
  static std::mt19937 rng(std::time(nullptr));
  std::uniform_int_distribution<uint8_t> dist(0, 255);
  return {dist(rng), dist(rng), dist(rng)};
}
}  // namespace random

std::string uuid();
std::string timestamp(std::time_t it);
inline std::string timestamp() {
  const auto now = std::chrono::system_clock::now();
  return timestamp(std::chrono::system_clock::to_time_t(now));
}

namespace ssha512 {
std::string sign(const std::vector<uint8_t> plain,
                 const std::vector<uint8_t> salt);
inline std::string sign(const std::vector<uint8_t> plain, size_t salt_len) {
  const auto salt = palm::random::bytes(salt_len);
  return sign(plain, salt);
}
inline std::string sign(const std::string& plain, size_t salt_len) {
  std::vector<uint8_t> it(plain.begin(), plain.end());
  return sign(it, salt_len);
}
inline std::string sign(const std::string& plain,
                        const std::vector<uint8_t> salt) {
  std::vector<uint8_t> it(plain.begin(), plain.end());
  return sign(it, salt);
}
bool verify(const std::string& code, const std::vector<uint8_t> plain);
inline bool verify(const std::string& code, const std::string& plain) {
  std::vector<uint8_t> it(plain.begin(), plain.end());
  return verify(code, it);
}
inline static const std::string HEADER = "{SSHA512}";
}  // namespace ssha512

namespace salted_password {
std::pair<std::string, double> verify(const std::string& code);
/*
js: Math.random()
*/
std::string sign(const std::string& plain);
}  // namespace salted_password
}  // namespace palm
