#pragma once

#include <sstream>
#include <stdexcept>
#include <string>

#include <boost/algorithm/string.hpp>

namespace palm {
namespace validator {
inline void email(const std::string& s) {
  // TODO
}
inline void phone(const std::string& s) {
  // TODO
}
inline void url(const std::string& s) {
  // TODO
}
inline void zipcode(const std::string& s) {
  // TODO
}
inline void address(const std::string& s) {
  // TODO
}
inline void string(const std::string& s, const size_t max, const size_t min) {
  const auto l = s.size();
  std::stringstream ss;
  if (l < min) {
    ss << "length can't lower than " << min;
    throw std::runtime_error(ss.str());
  }
  if (l > max) {
    ss << "length can't lager than " << max;
    throw std::runtime_error(ss.str());
  }
}
template <class T>
void range(const T val, const T max, const T min) {
  std::stringstream ss;
  if (val < min) {
    ss << "can't lower than " << min;
    throw std::runtime_error(ss.str());
  }
  if (val > max) {
    ss << "can't lager than " << max;
    throw std::runtime_error(ss.str());
  }
}
}  // namespace validator
}  // namespace palm
