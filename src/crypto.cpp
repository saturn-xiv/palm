#include "palm/crypto.hpp"

#include <algorithm>
#include <functional>
#include <random>

#include <boost/beast/core/detail/base64.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <Poco/JWT/Signer.h>
#include <Poco/JWT/Token.h>

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
