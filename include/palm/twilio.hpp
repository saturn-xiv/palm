// #pragma once

// #include "palm/env.hpp"

// namespace palm {

// namespace twilio {
// namespace voice {
// inline static const std::string API = "/api/twilio/voice";
// }

// namespace sms {
// inline static const std::string API = "/api/twilio/reply";
// struct Request {
//   std::string body;
//   std::string from;
//   std::string to;
// };
// NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Request, body, from, to)

// struct SubResourceUris {
//   std::string media;
// };
// NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(SubResourceUris, media)

// struct Response {
//   std::string account_sid;
//   std::string api_version;
//   std::string body;
//   std::string date_created;
//   std::optional<std::string> date_sent;
//   std::string date_updated;
//   std::string direction;
//   std::optional<int> error_code;
//   std::optional<std::string> error_message;
//   std::string from;
//   std::optional<std::string> messaging_service_sid;
//   std::string num_media;
//   std::string num_segments;
//   std::optional<std::string> price;
//   std::optional<std::string> price_unit;
//   std::string sid;
//   std::string status;
//   SubResourceUris subresource_uris;
//   std::string to;
//   std::string uri;
// };
// NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Response, account_sid, api_version, body,
//                                    date_created, date_sent, date_updated,
//                                    direction, error_code, error_message,
//                                    from, messaging_service_sid, num_media,
//                                    num_segments, price, price_unit, sid,
//                                    status, subresource_uris, to, uri)
// }  // namespace sms

// class Client {
//  public:
//   Client() {}
//   Client(const std::string& account_sid, const std::string& auth_token,
//          const std::string& from)
//       : account_sid(account_sid), auth_token(auth_token), from(from) {}

//   std::shared_ptr<palm::twilio::sms::Response> sms(const std::string& to,
//                                                    const std::string&
//                                                    message, bool callback =
//                                                    false);

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["account-sid"].value<std::string>();
//       if (it) {
//         this->account_sid = it.value();
//       }
//     }
//     {
//       auto it = node["auth-token"].value<std::string>();
//       if (it) {
//         this->auth_token = it.value();
//       }
//     }
//     {
//       auto it = node["from"].value<std::string>();
//       if (it) {
//         this->from = it.value();
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Client& self) {
//     node.insert_or_assign("account-sid", self.account_sid);
//     node.insert_or_assign("auth-token", self.auth_token);
//     node.insert_or_assign("from", self.from);

//     return node;
//   }

//  private:
//   std::string account_sid;
//   std::string auth_token;
//   std::string from;
//   inline const static std::string api_host = "https://api.twilio.com";
// };

// }  // namespace twilio
// }  // namespace palm
