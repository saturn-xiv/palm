#include "palm/validator.hpp"

#include <boost/algorithm/string.hpp>

std::optional<std::string> palm::validator::code(const std::string& s_) {
  std::string s = boost::algorithm::to_lower_copy(s_);
  boost::trim(s);
  const auto len = s.length();
  if (len < 2 || len > 127) {
    return std::nullopt;
  }
  return s;
}

std::optional<std::string> palm::validator::name(const std::string& s_) {
  std::string s = boost::trim_copy(s_);
  const auto len = s.length();
  if (len < 2 || len > 63) {
    return std::nullopt;
  }
  return s;
}
std::optional<std::string> palm::validator::email(const std::string& s_) {
  std::string s = boost::algorithm::to_lower_copy(s_);
  boost::trim(s);

  const auto len = s.length();
  if (len < 5 || len > 127) {
    return std::nullopt;
  }
  //   TODO valid email
  return s;
}
std::optional<std::string> palm::validator::password(const std::string& s) {
  const auto len = s.length();
  if (len < 6 || len > 32) {
    return std::nullopt;
  }
  return s;
}
