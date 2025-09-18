#pragma once

#include <optional>
#include <regex>
#include <string>

#include <hatn/validator/validator.hpp>
#include <hatn/validator/validate.hpp>

namespace palm {
namespace validator {
std::optional<std::string> code(const std::string& s);
std::optional<std::string> email(const std::string& s);
std::optional<std::string> name(const std::string& s);
std::optional<std::string> password(const std::string& s);
}  // namespace validator
}  // namespace palm
