#pragma once

#include <string>

#include <cpr/cpr.h>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace palm {
// https://www.twilio.com/docs/usage/api
class Twilio {
 public:
  Twilio(const toml::table& config)
      : _account_sid(config["account-sid"].value<std::string>().value()),
        _auth_token(config["auth-token"].value<std::string>().value()) {}
  Twilio(const std::string& account_sid, const std::string& auth_token)
      : _account_sid(account_sid), _auth_token(auth_token) {}

 private:
  std::string _account_sid;
  std::string _auth_token;
  inline static const std::string HOST = "https://api.twilio.com/2010-04-01/";
};
}  // namespace palm
