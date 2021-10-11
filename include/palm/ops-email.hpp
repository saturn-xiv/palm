#pragma once

#include "palm/env.hpp"

// https://www.linode.com/docs/guides/email-with-postfix-dovecot-and-mysql/
// https://mad9scientist.com/dovecot-password-creation-php/

namespace palm {
namespace ops {
namespace email {
namespace ssha512 {
const static std::string PASSWORD_HEADER = "{SHA512-CRYPT}";
const static std::string SALT_HEADER = "$6$";
const static size_t SALT_LEN = 16;
std::string sum(const std::string& plain);
bool verify(const std::string& password, const std::string& plain);
}  // namespace ssha512
}  // namespace email
}  // namespace ops
}  // namespace palm
