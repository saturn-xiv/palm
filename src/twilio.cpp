#include "palm/twilio.hpp"

#include <cpr/cpr.h>

// https://www.twilio.com/docs/usage/api
// https://support.twilio.com/hc/en-us/articles/223136047-Configure-a-Twilio-Phone-Number-to-Receive-and-Respond-to-Messages
// https://www.twilio.com/docs/sms/tutorials/how-to-confirm-delivery-java

// https://www.twilio.com/docs/usage/webhooks/sms-webhooks
// Your status delivery URL will receive an HTTP POST request with the
// application/x-www-form-urlencoded content type.
palm::twilio::sms::Response palm::twilio::Client::sms(
    const std::string& to, const std::string& message,
    const std::optional<std::string>& callback) {
  std::stringstream url;
  url << Client::api_host << "/2010-04-01/Accounts/" << this->account_sid
      << "/Messages.json";

  cpr::Payload payload = {{"From", this->from}, {"Body", message}, {"To", to}};

  if (callback) {
    payload.Add({"StatusCallback", callback.value()});
  }

  BOOST_LOG_TRIVIAL(info) << "send '" << message << "' to " << to;
  cpr::Response res = cpr::Post(
      cpr::Url{url.str()}, cpr::VerifySsl(false),
      cpr::Authentication{this->account_sid, this->auth_token}, payload);
  BOOST_LOG_TRIVIAL(info) << res.status_code << "(" << res.elapsed << "):\n"
                          << res.text;
  if (res.status_code == 201) {
    const auto js = nlohmann::json::parse(res.text);
    palm::twilio::sms::Response it = js.get<palm::twilio::sms::Response>();
    return it;
  }

  throw std::invalid_argument(res.reason);
}
