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

BOOST_AUTO_TEST_CASE(jwt) {}

BOOST_AUTO_TEST_CASE(aes) {}

BOOST_AUTO_TEST_CASE(hs512) {}

BOOST_AUTO_TEST_CASE(ssha512) {}
