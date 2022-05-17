#pragma once

#include <chrono>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include <boost/property_tree/ptree.hpp>

namespace palm {

std::string uuid();

namespace random {
std::string alphanumeric(const size_t len);
std::vector<uint8_t> bytes(const size_t len);
}  // namespace random

namespace base64 {
std::string encode(const std::vector<uint8_t>& buf);
std::vector<uint8_t> decode(const std::string& str);
}  // namespace base64

class Jwt {
 public:
  Jwt(const boost::property_tree::ptree& config);
  Jwt(const std::string& key) : key(key) {}
  std::string sign(
      const std::string& subject,
      const std::unordered_map<std::string, std::string>& payload,
      const std::chrono::seconds& ttl = std::chrono::days(1)) const;
  std::pair<std::string, std::unordered_map<std::string, std::string>> verify(
      const std::string& token) const;

 private:
  std::string key;
};
}  // namespace palm
