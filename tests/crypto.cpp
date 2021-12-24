#define BOOST_TEST_MODULE crypto
#include <boost/test/unit_test.hpp>

#include "palm/crypto.hpp"

#include <iostream>

BOOST_AUTO_TEST_CASE(random_) {
  for (int i = 0; i < 10; i++) {
    std::cout << "--------- " << i << " --------" << std::endl;
    std::cout << "uuid:\t" << palm::uuid() << std::endl;
    std::cout << "bytes:\t" << palm::base64::to(palm::random::bytes(32))
              << std::endl;
    std::cout << "string:\t" << palm::random::string(16) << std::endl;
  }
}

#define PALM_TEST_SECRET_KEY "ticn2mE1i+TUZw9wIUbb2FjYzRfgaNWpijq7tqNzmAY="

BOOST_AUTO_TEST_CASE(hmac_512) {
  palm::Hmac it(PALM_TEST_SECRET_KEY);
  for (int i = 0; i < 10; i++) {
    const auto data1 = palm::random::bytes(64);
    const auto data2 = palm::random::bytes(64);
    const auto salt1 = palm::random::bytes(8);
    const auto salt2 = palm::random::bytes(8);
    const auto hs11 = palm::base64::to(it.sha512(data1, salt1));
    const auto hs21 = palm::base64::to(it.sha512(data2, salt1));
    const auto hs12 = palm::base64::to(it.sha512(data1, salt2));
    const auto hs11_0 = palm::base64::to(it.sha512(data1, salt1));

    std::cout << "data 1: " << palm::base64::to(data1)
              << "\ndata 2: " << palm::base64::to(data2)
              << "\nsalt 1: " << palm::base64::to(salt1)
              << "\nsalt 2: " << palm::base64::to(salt2)
              << "\n sum 1 1: " << hs11 << "\n sum 2 1: " << hs21
              << "\n sum 1 2: " << hs12 << "\n sum 1 1(r): " << hs11_0
              << std::endl;

    BOOST_TEST(hs11 != hs12);
    BOOST_TEST(hs21 != hs12);
    BOOST_TEST(hs11 == hs11_0);
  }
}

BOOST_AUTO_TEST_CASE(jwt) {}

BOOST_AUTO_TEST_CASE(aes) {}

BOOST_AUTO_TEST_CASE(hs512) {}

BOOST_AUTO_TEST_CASE(ssha512) {}
