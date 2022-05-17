#include "palm/crypto.hpp"

#include <algorithm>
#include <functional>
#include <random>

#include <boost/beast/core/detail/base64.hpp>
#include <boost/log/trivial.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <Poco/Base64Decoder.h>
#include <Poco/Base64Encoder.h>
#include <Poco/HMACEngine.h>
#include <Poco/JWT/Signer.h>
#include <Poco/JWT/Token.h>
#include <Poco/SHA2Engine.h>

std::string palm::uuid() {
  thread_local static auto gen = boost::uuids::random_generator();
  boost::uuids::uuid it = gen();
  return boost::uuids::to_string(it);
}

std::string palm::random::alphanumeric(const size_t len) {
  static constexpr auto chars = "0123456789abcdefghijklmnopqrstuvwxyz";
  thread_local static std::mt19937 rng{std::random_device{}()};
  thread_local static std::uniform_int_distribution<std::string::size_type>
      dist(0, std::strlen(chars) - 1);
  auto result = std::string(len, '\0');
  std::generate_n(begin(result), len, [&]() { return chars[dist(rng)]; });
  return result;
}

std::vector<uint8_t> palm::random::bytes(const size_t len) {
  thread_local static std::independent_bits_engine<std::default_random_engine,
                                                   CHAR_BIT, uint8_t>
      rng;
  std::vector<uint8_t> buf(len);
  std::generate(std::begin(buf), std::end(buf), std::ref(rng));
  return buf;
}

std::string palm::base64::encode(const std::vector<uint8_t>& buf) {
  std::string str;
  str.resize(boost::beast::detail::base64::encoded_size(buf.size()));
  std::size_t read = boost::beast::detail::base64::encode(
      &str.front(), buf.data(), buf.size());
  str.resize(read);
  return str;
}

std::vector<uint8_t> palm::base64::decode(const std::string& str) {
  std::vector<uint8_t> buf;
  buf.resize(boost::beast::detail::base64::decoded_size(str.size()));
  std::size_t read =
      boost::beast::detail::base64::decode(buf.data(), str.c_str(), str.size())
          .first;
  buf.resize(read);
  return buf;
}

palm::Jwt::Jwt(const boost::property_tree::ptree& config) {
  this->key = config.get<std::string>("jwt.secret-key");
}

std::string palm::Jwt::sign(
    const std::string& subject,
    const std::unordered_map<std::string, std::string>& payload,
    const std::chrono::seconds& ttl) const {
  Poco::JWT::Token token;
  token.setType("JWT");
  token.setSubject(subject);

  for (const auto& [k, v] : payload) {
    token.payload().set(k, v);
  }
  const auto now = Poco::Timestamp();
  //   token.setIssuedAt(now);
  token.setNotBefore(now);
  token.setExpiration(
      now + std::chrono::duration_cast<std::chrono::microseconds>(ttl).count());
  Poco::JWT::Signer signer(this->key);
  return signer.sign(token, Poco::JWT::Signer::ALGO_HS512);
}

std::pair<std::string, std::unordered_map<std::string, std::string>>
palm::Jwt::verify(const std::string& token) const {
  Poco::JWT::Signer signer(this->key);
  signer.addAlgorithm(Poco::JWT::Signer::ALGO_HS512);
  Poco::JWT::Token jwt = signer.verify(token);

  std::string subject = jwt.getSubject();
  std::unordered_map<std::string, std::string> payload;

  std::vector<std::string> keys;
  jwt.payload().getNames(keys);
  for (const auto& k : keys) {
    const auto v = jwt.payload().getValue<std::string>(k);
    payload[k] = v;
  }
  return std::make_pair(subject, payload);
}

std::vector<uint8_t> palm::HMac::sign(const std::vector<uint8_t>& plain,
                                      const size_t salt_len) const {
  const auto salt = palm::random::bytes(salt_len);
  return this->sign(plain, salt);
}
bool palm::HMac::verify(const std::vector<uint8_t>& code,
                        const std::vector<uint8_t>& plain) const {
  if (code.size() <= SHA512_DIGEST_LENGTH) {
    return false;
  }
  const std::vector<uint8_t> salt = {code.begin() + SHA512_DIGEST_LENGTH,
                                     code.end()};

  const auto buf = this->sign(plain, salt);
  return buf == code;
}

std::vector<uint8_t> palm::HMac::sign(const std::vector<uint8_t>& plain,
                                      const std::vector<uint8_t>& salt) const {
  Poco::HMACEngine<palm::SHA512Engine> eng(this->key);
  eng.update(&*plain.begin(), plain.size());
  eng.update(&*salt.begin(), salt.size());
  const auto hash = eng.digest();
  std::vector<uint8_t> buf;
  {
    buf.insert(buf.end(), hash.begin(), hash.end());
    buf.insert(buf.end(), salt.begin(), salt.end());
  }
  return buf;
}

std::string palm::ssha512::sign(const std::string& plain,
                                const size_t salt_len) {
  const auto salt = palm::random::alphanumeric(salt_len);
  return palm::ssha512::sign(plain, salt);
}
bool palm::ssha512::verify(const std::string& code, const std::string& plain) {
  if (code.size() <= HEADER.size()) {
    return false;
  }

  std::ostringstream os;
  {
    const std::string body = code.substr(HEADER.size());
    std::istringstream is(body);
    Poco::Base64Decoder decoder(is);
    std::copy(std::istreambuf_iterator<char>(decoder),
              std::istreambuf_iterator<char>(),
              std::ostreambuf_iterator<char>(os));
  }

  const std::string buf = os.str();
  if (buf.size() <= SHA512_DIGEST_LENGTH) {
    return false;
  }
  const std::string salt = buf.substr(SHA512_DIGEST_LENGTH);
  return palm::ssha512::sign(plain, salt) == code;
}
std::string palm::ssha512::sign(const std::string& plain,
                                const std::string& salt) {
  Poco::SHA2Engine eng(Poco::SHA2Engine::ALGORITHM::SHA_512);
  eng.update(plain);
  eng.update(salt);
  const auto hash = eng.digest();
  std::vector<uint8_t> buf;
  {
    buf.insert(buf.end(), hash.begin(), hash.end());
    buf.insert(buf.end(), salt.begin(), salt.end());
  }
  std::ostringstream ss;
  ss << HEADER;

  {
    Poco::Base64Encoder encoder(ss);
    encoder.write(reinterpret_cast<const char*>(&*buf.begin()), buf.size());
    encoder.close();
  }

  return ss.str();
}
