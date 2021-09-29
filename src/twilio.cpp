// #include "palm/twilio.hpp"
// #include "palm/theme.hpp"

// // https://www.twilio.com/docs/usage/api
// //
// https://support.twilio.com/hc/en-us/articles/223136047-Configure-a-Twilio-Phone-Number-to-Receive-and-Respond-to-Messages
// // https://www.twilio.com/docs/sms/tutorials/how-to-confirm-delivery-java

// // https://www.twilio.com/docs/usage/webhooks/sms-webhooks
// // Your status delivery URL will receive an HTTP POST request with the
// // application/x-www-form-urlencoded content type.
// std::shared_ptr<palm::twilio::sms::Response> palm::twilio::Client::sms(
//     const std::string& to, const std::string& message, bool callback) {
//   httplib::Client cli(Client::api_host.c_str());
//   cli.enable_server_certificate_verification(false);
//   cli.set_basic_auth(this->account_sid.c_str(), this->auth_token.c_str());
//   httplib::Params form = {
//       {"From", this->from},
//       {"Body", message.c_str()},
//       {"To", to.c_str()},
//   };
//   auto res = cli.Post(
//       ("/2010-04-01/Accounts/" + this->account_sid +
//       "/Messages.json").c_str(), form);
//   palm::log(res);
//   if (res->status == 200 || res->status == 201) {
//     const auto js = nlohmann::json::parse(res->body);
//     const auto it = std::make_shared<palm::twilio::sms::Response>();
//     *it = js.get<palm::twilio::sms::Response>();
//     return it;
//   }

//   throw std::invalid_argument(res->reason);
// }
