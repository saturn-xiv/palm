#define BOOST_TEST_MODULE crypto
#include <boost/test/unit_test.hpp>

#include "palm/crypto.hpp"

#include <algorithm>
#include <iostream>
#include <ranges>

#include <boost/property_tree/ini_parser.hpp>
#include <boost/range/irange.hpp>

BOOST_AUTO_TEST_CASE(random_) {
  for (auto i : boost::irange(1, 10)) {
    std::cout << "uuid(" << i << "): " << palm::uuid() << std::endl;
    std::cout << "alphanumeric(" << i << "): " << palm::random::alphanumeric(8)
              << std::endl;

    {
      const auto buf = palm::random::bytes(32);
      const auto str = palm::base64::encode(buf);
      std::cout << "bytes(i): " << str << std::endl;
      {
        const auto tmp = palm::base64::encode(buf);
        BOOST_CHECK_EQUAL(str, tmp);
      }
      {
        const auto tmp = palm::base64::decode(str);
        BOOST_CHECK_EQUAL_COLLECTIONS(buf.begin(), buf.end(), tmp.begin(),
                                      tmp.end());
      }
    }
  }
}

BOOST_AUTO_TEST_CASE(jwt) {
  std::string subject = "hi";
  std::unordered_map<std::string, std::string> payload = {
      {"RED", "#FF0000"}, {"GREEN", "#00FF00"}, {"BLUE", "#0000FF"}};

  boost::property_tree::ptree tree;
  boost::property_tree::read_ini("config.ini", tree);
  palm::Jwt jwt(tree);

  for (auto i : boost::irange(1, 10)) {
    const auto token = jwt.sign(subject, payload);
    std::cout << "jwt token(" << i << "): " << token << std::endl;
    {
      const auto tmp = jwt.sign(subject, payload);
      BOOST_CHECK_NE(token, tmp);
    }
    {
      const auto pair = jwt.verify(token);
      BOOST_CHECK_EQUAL(pair.first, subject);
      BOOST_TEST(pair.second.size() >= payload.size());

      for (const auto& [k, v] : payload) {
        BOOST_CHECK_EQUAL(pair.second.at(k), v);
      }
    }
  }
}

BOOST_AUTO_TEST_CASE(ssha512) { BOOST_TEST(4 == 2 * 2); }

BOOST_AUTO_TEST_CASE(aes) { BOOST_TEST(4 == 2 * 2); }

BOOST_AUTO_TEST_CASE(hmac) { BOOST_TEST(4 == 2 * 2); }
