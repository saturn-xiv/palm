#define BOOST_TEST_MODULE encrypt
#include <boost/test/included/unit_test.hpp>

#include "palm/crypto.hpp"
#include "palm/jwt.hpp"
#include "palm/utils.hpp"

#include <jwt-cpp/jwt.h>

#define PALM_SALT_SIZE 12
#define PALM_LOOP_SIZE 6

BOOST_AUTO_TEST_CASE(random_) {
  {
    const auto it = palm::timestamp();
    std::cout << "Current Timestamp: " << it << std::endl;
    BOOST_CHECK_EQUAL(it.size(), 14);
  }

  for (int i = 1; i < PALM_LOOP_SIZE; i++) {
    std::cout << "UUID(" << i << "): " << palm::uuid() << std::endl;
  }
  {
    for (int i = 1; i < PALM_LOOP_SIZE; i++) {
      const auto buf = palm::random::bytes(PALM_SALT_SIZE);
      BOOST_CHECK_EQUAL(buf.size(), PALM_SALT_SIZE);
      const auto it = palm::base64::to_string(buf);
      std::cout << "random bytes(" << i << "): " << it << std::endl;
      {
        const auto tmp = palm::base64::from_string(it);
        BOOST_CHECK_EQUAL_COLLECTIONS(buf.begin(), buf.end(), tmp.begin(),
                                      tmp.end());
      }
    }
  }
  {
    for (int i = 1; i < PALM_LOOP_SIZE; i++) {
      const auto it = palm::random::alphanumeric(PALM_SALT_SIZE);
      std::cout << "rand alphanumeric(" << i << "): " << it << std::endl;
      BOOST_CHECK_EQUAL(it.size(), PALM_SALT_SIZE);
    }
  }
}

BOOST_AUTO_TEST_CASE(sha) {
  const std::string hi = "Hello, palm!";
  {
    const auto val = palm::sha256::sign(hi);
    {
      const auto tmp = palm::sha256::sign(hi);
      BOOST_CHECK_EQUAL_COLLECTIONS(val.begin(), val.end(), tmp.begin(),
                                    tmp.end());
    }

    {
      const auto hash = palm::base64::to_string(val);
      std::cout << "sha256('" << hi << "'): " << hash << std::endl;
      BOOST_CHECK_NE(hash, "");
      BOOST_CHECK_NE(hash, hi);
    }
  }
  {
    const auto val = palm::sha512::sign(hi);
    {
      const auto tmp = palm::sha512::sign(hi);
      BOOST_CHECK_EQUAL_COLLECTIONS(val.begin(), val.end(), tmp.begin(),
                                    tmp.end());
    }
    {
      const auto hash = palm::base64::to_string(val);
      std::cout << "sha512('" << hi << "'): " << hash << std::endl;
      BOOST_CHECK_NE(hash, "");
      BOOST_CHECK_NE(hash, hi);
    }
  }
}

// https://docs.gravatar.com/api/avatars/hash/
BOOST_AUTO_TEST_CASE(gravatar) {
  for (const auto& it :
       {" MyEmailAddress@example.com", "MyEmailAddress@example.com ",
        " MyEmailAddress@example.com ", "MyEmailAddress@example.com",
        "myemailaddress@example.com"}) {
    const auto iv = palm::gravatar::hash(it);
    BOOST_CHECK_EQUAL(
        iv, "84059b07d4be67b806386c0aad8070a23f18836bbaae342275dc0a83414c32ee");
  }
}

// https://mad9scientist.com/dovecot-password-creation-php/
// https://wiki.archlinux.org/title/Dovecot
BOOST_AUTO_TEST_CASE(ssha512) {
  const std::string hi = "Hello, palm!";
  const size_t salt_len = 8;
  const auto hash = palm::ssha512::sign(hi, salt_len);

  BOOST_CHECK_NE(hash, "");
  BOOST_CHECK_NE(hash, hi);
  BOOST_CHECK_NE(hash, palm::ssha512::sign(hi, salt_len));

  std::cout << "doveadm pw -t '" << hash << "' -p '" << hi << "'" << std::endl;
  BOOST_CHECK(palm::ssha512::verify(hash, hi));
}

BOOST_AUTO_TEST_CASE(hmac) {
  const palm::HMac mac("N1wwoQI4JFBgc4H1a54eaehu3LMAg7aaUssQm1bIbV8=");
  const std::string hi = "Hello, palm!";
  {
    const auto val = mac.sign(hi);
    {
      const auto hash = palm::base64::to_string(val);
      BOOST_CHECK_NE(hash, "");
      BOOST_CHECK_NE(hash, hi);
      std::cout << "hmac('" << hi << "'): " << hash << std::endl;
    }
    {
      const auto tmp = mac.sign(hi);
      BOOST_CHECK_EQUAL_COLLECTIONS(val.begin(), val.end(), tmp.begin(),
                                    tmp.end());
    }
    BOOST_CHECK(mac.verify(val, hi));
  }
}

BOOST_AUTO_TEST_CASE(aes) {
  const palm::Aes aes("CHPQawkOBdyOecCeT3CTkQCp4/4sJ6F3rXoqwc3XOD8=",
                      "vLKjAvdMAO5th+6ytUKCWQ==");
  const std::string hi = "Hello, palm!";
  {
    const auto val = aes.encrypt(hi);
    {
      const auto code = palm::base64::to_string(val);
      BOOST_CHECK_NE(code, "");
      BOOST_CHECK_NE(code, hi);
      std::cout << "aes('" << hi << "'): " << code << std::endl;
    }
    {
      const auto tmp = aes.encrypt(hi);
      BOOST_CHECK_EQUAL_COLLECTIONS(val.begin(), val.end(), tmp.begin(),
                                    tmp.end());
    }
    {
      const auto tmp = aes.decrypt(val);
      std::string hello(tmp.begin(), tmp.end());
      BOOST_CHECK_EQUAL(hello, hi);
    }
  }
}

// https://www.epochconverter.com/
BOOST_AUTO_TEST_CASE(jwt_) {
  const std::string jwt_id = "jjj";
  const std::string key_id = "kkk";
  const std::string audience_1 = "a1";
  const std::string audience_2 = "a2";
  const std::set<std::string> audiences = {audience_1, audience_2, "a3"};
  const std::string issuer = "iii";
  const std::string subject = "sss";
  const std::string payload = "ppp";

  palm::Jwt jwt(
      "ieyeZo1thohc3oojaidoh3Aik1iDaht4iuX4ahvoh3mungah8iechahf2eim0noo");
  {
    const auto token = jwt.sign(issuer, subject, audiences, payload);
    std::cout << "jwt token: " << token << std::endl;

    {
      palm::Jwt tmp(
          "pheefeepah9phieme0JohP7soh6phah7aefohPaicei6oom1eidooghoSuno2aSo");

      BOOST_REQUIRE_THROW(tmp.verify(token, issuer, audience_1),
                          jwt::error::signature_verification_exception);
    }
    {
      const auto [jid, kid, sub, pay] = jwt.verify(token, issuer, audience_2);
      BOOST_CHECK(!jid.has_value());
      BOOST_CHECK(!kid.has_value());
      BOOST_CHECK_EQUAL(sub, subject);
      BOOST_REQUIRE(pay.has_value());
      BOOST_CHECK_EQUAL(pay.value(), payload);
    }
    BOOST_REQUIRE_THROW(jwt.verify(token, issuer, "t0"),
                        jwt::error::signature_verification_exception);
  }
}
