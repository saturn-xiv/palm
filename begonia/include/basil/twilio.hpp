#pragma once

#include <string>

#include <cpr/cpr.h>
#include <toml++/toml.hpp>

namespace basil {
// https://www.twilio.com/docs/usage/api
class Twilio {
 public:
  Twilio(const std::string& account_sid, const std::string& auth_token)
      : _account_sid(account_sid), _auth_token(auth_token) {}

 private:
  std::string _account_sid;
  std::string _auth_token;
  inline static const std::string HOST = "https://api.twilio.com/2010-04-01/";
};
}  // namespace basil
