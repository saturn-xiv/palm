#define BOOST_TEST_MODULE crypto
#include <boost/test/included/unit_test.hpp>

// #include "palm/cache.hpp"
// #include "palm/crypto.hpp"
// #include "palm/ops-email.hpp"

// static const size_t PALM_LOOP_COUNT = 10;

// // openssl rand -base64 32
// static const std::string PALM_SECRETS_KEY =
//     "k5XdcOND2+2oU7dUFDalP6yei0qcKJLaScJTI8NMYKk=";

BOOST_AUTO_TEST_CASE(uuid) {
  // for (auto i = 0; i < PALM_LOOP_COUNT; i++) {
  //   std::cout << palm::uuid::v4() << std::endl;
  // }
}

BOOST_AUTO_TEST_CASE(string) {
  // for (auto i = 0; i < PALM_LOOP_COUNT; i++) {
  //   std::cout << palm::random::string(8) << std::endl;
  // }
}

BOOST_AUTO_TEST_CASE(bytes) {
  // for (auto i = 0; i < PALM_LOOP_COUNT; i++) {
  //   std::cout << palm::base64::encode(palm::random::bytes(16)) << std::endl;
  // }
}

BOOST_AUTO_TEST_CASE(url) {
  // for (const auto& it :
  //      {"?", "&", "+", ")", "\"", "(", "|", "=", "\n", "\t", " "}) {
  //   std::stringstream ss;
  //   ss << "aa" << it << "bb";
  //   const auto s = ss.str();
  //   std::cout << s << "\t" << palm::url::encode(s) << std::endl;
  // }
}

BOOST_AUTO_TEST_CASE(ssha512) {
  // const std::string plain = "hi, palm!";
  // std::cout << "doveadm pw -s SHA512-CRYPT -p '" << plain << "' -r 5000"
  //           << std::endl;
  // for (auto i = 0; i < PALM_LOOP_COUNT; i++) {
  //   const auto password = palm::ops::email::ssha512::sum(plain.c_str());
  //   std::cout << "doveadm pw -t '" << password << "' -p '" << plain << "'"
  //             << std::endl;
  //   BOOST_CHECK(palm::ops::email::ssha512::verify(password, plain));
  //   BOOST_CHECK(!palm::ops::email::ssha512::verify(password, "hi"));
  // }
}

BOOST_AUTO_TEST_CASE(hmac) {
  // const std::string plain = "hi, palm!";
  // const palm::Hmac hmac(PALM_SECRETS_KEY);
  // for (auto i = 0; i < 10; i++) {
  //   const auto password = hmac.sum(plain, 8);
  //   BOOST_CHECK(hmac.verify(password, plain));
  //   BOOST_CHECK(!hmac.verify(password, "hi"));
  // }
}

BOOST_AUTO_TEST_CASE(aes) {
  //   const std::string plain =
  //       R"V0G0N(OpenSSL uses PKCS7 padding by default.
  // This padding means when your data is not a multiple of the block size, you
  // pad n bytes of the value n, where n is however many bytes you need to get
  // to the block size. AES's block size is 16.)V0G0N";
  //   const palm::Aes aes(PALM_SECRETS_KEY);
  //   for (auto i = 0; i < 10; i++) {
  //     const auto code = aes.encrypt(plain);
  //     const auto tmp = aes.decrypt(code.first, code.second);
  //     std::cout << plain << " vs " << tmp << "(" << code.first.size() << ","
  //               << code.second.size() << ")" << std::endl;
  //     BOOST_CHECK_EQUAL(tmp, plain);
  //     BOOST_CHECK(tmp != "hi");
  //   }
}

BOOST_AUTO_TEST_CASE(jwt) {
  // const auto issuer = "who-am-i";
  // const palm::Jwt jwt(PALM_SECRETS_KEY, issuer);

  // const palm::Jwt er1("111222333", issuer);
  // const palm::Jwt er2(PALM_SECRETS_KEY, "2");

  // for (auto i = 0; i < 10; i++) {
  //   std::unordered_map<std::string, std::string> it = {
  //       {"RED", "#FF0000"}, {"GREEN", "#00FF00"}, {"BLUE", "#0000FF"}};

  //   {
  //     std::stringstream ss;
  //     ss << "Hi, user " << i + 1;
  //     it[palm::Session::UID] = ss.str();
  //   }

  //   {
  //     const auto token = jwt.sum(it, std::chrono::seconds{-1});
  //     BOOST_CHECK_THROW(jwt.parse(token), std::system_error);
  //   }

  //   const auto token = jwt.sum(it, std::chrono::hours{3});
  //   BOOST_CHECK_THROW(er1.parse(token), std::system_error);
  //   BOOST_CHECK_THROW(er2.parse(token), std::system_error);

  //   const auto tmp = jwt.parse(token);

  //   std::cout << token << std::endl;

  //   for (const auto& it : tmp) {
  //     std::cout << it.first << "\t" << it.second << std::endl;
  //   }
  //   BOOST_CHECK(it.size() <= tmp.size());
  // }
}
