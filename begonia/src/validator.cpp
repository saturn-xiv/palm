#include "palm/validator.hpp"

#include <boost/algorithm/string.hpp>

#include <hatn/validator/validate.hpp>
#include <hatn/validator/validator.hpp>

std::optional<std::string> palm::validator::code(const std::string& s_) {
  std::string s = boost::algorithm::to_lower_copy(s_);
  boost::trim(s);
  // TODO
  auto v = HATN_VALIDATOR_NAMESPACE::validator(
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::gte, 2),
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::lte, 63));
  if (!v.apply(s)) {
    return std::nullopt;
  }

  return s;
}

std::optional<std::string> palm::validator::name(const std::string& s_) {
  std::string s = boost::trim_copy(s_);
  auto v = HATN_VALIDATOR_NAMESPACE::validator(
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::gte, 2),
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::lte, 63));
  if (!v.apply(s)) {
    return std::nullopt;
  }
  return s;
}
std::optional<std::string> palm::validator::email(const std::string& s_) {
  std::string s = boost::algorithm::to_lower_copy(s_);
  boost::trim(s);
  // TODO
  auto v = HATN_VALIDATOR_NAMESPACE::validator(
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::gte, 5),
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::lte, 127));
  if (!v.apply(s)) {
    return std::nullopt;
  }
  return s;
}
std::optional<std::string> palm::validator::password(const std::string& s) {
  const auto len = s.length();
  // TODO
  auto v = HATN_VALIDATOR_NAMESPACE::validator(
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::gte, 6),
      HATN_VALIDATOR_NAMESPACE::size(HATN_VALIDATOR_NAMESPACE::lte, 32));
  if (!v.apply(s)) {
    return std::nullopt;
  }
  return s;
}

static std::regex gl_ipv4_regex(
    R"RE(^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$)RE");
std::optional<std::string> palm::validator::ipv4(const std::string& s) {
  if (!std::regex_match(s, gl_ipv4_regex)) {
    return std::nullopt;
  }
  return s;
}
