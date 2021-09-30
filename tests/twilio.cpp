#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

TEST_CASE("Demo", "[f]") { REQUIRE(2 * 2 == 4); }

// #define BOOST_TEST_MODULE twilio
// #include <boost/test/included/unit_test.hpp>

// // #include "palm/twilio.hpp"

// BOOST_AUTO_TEST_CASE(model) {
//   // {
//   //   palm::twilio::sms::Request req;
//   //   nlohmann::json js = req;
//   //   std::cout << js.dump() << std::endl;
//   // }
//   // {
//   //   palm::twilio::sms::Response res;
//   //   nlohmann::json js = res;
//   //   std::cout << js.dump() << std::endl;
//   // }
// }

// // curl -X POST https://api.twilio.com/2010-04-01/Accounts/$TWILIO_ACCOUNT_SID/Messages.json \
// // --data-urlencode "Body=This will be the body of the new message." \
// // --data-urlencode "From=$TWILIO_FROM" \
// // --data-urlencode "To=$TWILIO_TO" \
// // -u $TWILIO_ACCOUNT_SID:$TWILIO_AUTH_TOKEN
// BOOST_AUTO_TEST_CASE(sms) {
//   // palm::twilio::Client cli(std::getenv("TWILIO_ACCOUNT_SID"),
//   //                          std::getenv("TWILIO_AUTH_TOKEN"),
//   //                          std::getenv("TWILIO_FROM"));
//   // {
//   //   const auto res = cli.sms(std::getenv("TWILIO_TO"), "Hello, Palm!");

//   //   std::cout << "return url: " << res->uri << std::endl;
//   // }
// }
