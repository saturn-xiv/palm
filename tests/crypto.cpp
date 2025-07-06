#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers.hpp>
#include <catch2/matchers/catch_matchers_vector.hpp>

#include "palm/crypto.hpp"
#include "palm/jwt.hpp"
#include "palm/utils.hpp"

#include <jwt-cpp/jwt.h>

#define PALM_SALT_SIZE 12
#define PALM_LOOP_SIZE 6

TEST_CASE("random data", "[random]") {
  {
    const auto it = palm::timestamp();
    std::cout << "Current Timestamp: " << it << std::endl;
    REQUIRE(it.size() == 14);
  }

  for (int i = 1; i < PALM_LOOP_SIZE; i++) {
    std::cout << "UUID(" << i << "): " << palm::uuid() << std::endl;
  }
  {
    for (int i = 1; i < PALM_LOOP_SIZE; i++) {
      const auto buf = palm::random::bytes(PALM_SALT_SIZE);
      REQUIRE(buf.size() == PALM_SALT_SIZE);
      const auto it = palm::base64::to_string(buf);
      std::cout << "random bytes(" << i << "): " << it << std::endl;
      {
        const auto tmp = palm::base64::from_string(it);
        REQUIRE_THAT(buf, Catch::Matchers::Equals(tmp));
      }
    }
  }
  {
    for (int i = 1; i < PALM_LOOP_SIZE; i++) {
      const auto it = palm::random::alphanumeric(PALM_SALT_SIZE);
      std::cout << "rand alphanumeric(" << i << "): " << it << std::endl;
      REQUIRE(it.size() == PALM_SALT_SIZE);
    }
  }
}

TEST_CASE("impl by openssl", "[sha]") {
  const std::string hi = "Hello, palm!";

  SECTION("sha256") {
    const auto val = palm::sha256::sign(hi);
    {
      const auto tmp = palm::sha256::sign(hi);
      REQUIRE_THAT(val, Catch::Matchers::Equals(tmp));
    }

    {
      const auto hash = palm::base64::to_string(val);
      std::cout << "sha256('" << hi << "'): " << hash << std::endl;
      REQUIRE(hash != "");
      REQUIRE(hash != hi);
    }
  }

  SECTION("sha512") {
    const auto val = palm::sha512::sign(hi);
    {
      const auto tmp = palm::sha512::sign(hi);
      REQUIRE_THAT(val, Catch::Matchers::Equals(tmp));
    }
    {
      const auto hash = palm::base64::to_string(val);
      std::cout << "sha512('" << hi << "'): " << hash << std::endl;
      REQUIRE(hash != "");
      REQUIRE(hash != hi);
    }
  }
}

// https://docs.gravatar.com/api/avatars/hash/
TEST_CASE("gravator profile", "[gravatar]") {
  for (const auto& it :
       {" MyEmailAddress@example.com", "MyEmailAddress@example.com ",
        " MyEmailAddress@example.com ", "MyEmailAddress@example.com",
        "myemailaddress@example.com"}) {
    const auto iv = palm::gravatar::hash(it);
    REQUIRE(iv ==
            "84059b07d4be67b806386c0aad8070a23f18836bbaae342275dc0a83414c32ee");
  }
}

// https://mad9scientist.com/dovecot-password-creation-php/
// https://wiki.archlinux.org/title/Dovecot
TEST_CASE("impl by openssl", "[ssha]") {
  const std::string hi = "Hello, palm!";
  const size_t salt_len = 8;

  SECTION("ssha512") {
    const auto hash = palm::ssha512::sign(hi, salt_len);

    REQUIRE(hash != "");
    REQUIRE(hash != hi);
    REQUIRE(hash != palm::ssha512::sign(hi, salt_len));

    std::cout << "doveadm pw -t '" << hash << "' -p '" << hi << "'"
              << std::endl;
    REQUIRE(palm::ssha512::verify(hash, hi));
  }
}

TEST_CASE("impl by openssl", "[hmac]") {
  const palm::HMac mac("N1wwoQI4JFBgc4H1a54eaehu3LMAg7aaUssQm1bIbV8=");
  const std::string hi = "Hello, palm!";
  {
    const auto val = mac.sign(hi);
    {
      const auto hash = palm::base64::to_string(val);
      REQUIRE(hash != "");
      REQUIRE(hash != hi);
      std::cout << "hmac('" << hi << "'): " << hash << std::endl;
    }
    {
      const auto tmp = mac.sign(hi);
      REQUIRE_THAT(val, Catch::Matchers::Equals(tmp));
    }
    REQUIRE(mac.verify(val, hi));
  }
}

TEST_CASE("impl by openssl", "[aes]") {
  const palm::Aes aes("CHPQawkOBdyOecCeT3CTkQCp4/4sJ6F3rXoqwc3XOD8=",
                      "vLKjAvdMAO5th+6ytUKCWQ==");
  const std::string hi = "Hello, palm!";
  {
    const auto val = aes.encrypt(hi);
    {
      const auto code = palm::base64::to_string(val);
      REQUIRE(code != "");
      REQUIRE(code != hi);
      std::cout << "aes('" << hi << "'): " << code << std::endl;
    }
    {
      const auto tmp = aes.encrypt(hi);
      REQUIRE_THAT(val, Catch::Matchers::Equals(tmp));
    }
    {
      const auto tmp = aes.decrypt(val);
      std::string hello(tmp.begin(), tmp.end());
      REQUIRE(hello == hi);
    }
  }
}

// https://www.epochconverter.com/
TEST_CASE("impl by openssl", "[jwt]") {
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

      REQUIRE_THROWS_AS(tmp.verify(token, issuer, audience_1),
                        jwt::error::signature_verification_exception);
    }
    {
      const auto [jid, kid, sub, pay] = jwt.verify(token, issuer, audience_2);
      REQUIRE(!jid.has_value());
      REQUIRE(!kid.has_value());
      REQUIRE(sub == subject);
      REQUIRE(pay.has_value());
      REQUIRE(pay.value() == payload);
    }
    REQUIRE_THROWS_AS(jwt.verify(token, issuer, "t0"),
                      jwt::error::signature_verification_exception);
  }
}
