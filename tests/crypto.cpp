#define BOOST_TEST_MODULE crypto
#include <boost/test/unit_test.hpp>

#include "palm/crypto.hpp"

#include <iostream>

BOOST_AUTO_TEST_CASE(random_) {
  for (int i = 0; i < 10; i++) {
    std::cout << "UUID: " << palm::uuid() << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(jwt) {}

BOOST_AUTO_TEST_CASE(aes) {}

BOOST_AUTO_TEST_CASE(hs512) {}

BOOST_AUTO_TEST_CASE(ssha512) {}
