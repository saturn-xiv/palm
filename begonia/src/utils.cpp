#include "palm/utils.hpp"
#include "palm/crypto.hpp"

#include <fstream>

#include <boost/algorithm/hex.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/algorithm/string/join.hpp>

// https://docs.gravatar.com/api/avatars/hash/
std::string palm::gravatar::hash(const std::string& email) {
  std::string e = boost::trim_copy(email);
  boost::algorithm::to_lower(e);
  auto d = palm::sha256::sign(e);

  std::string h;
  boost::algorithm::hex(d.begin(), d.end(), std::back_inserter(h));
  boost::algorithm::to_lower(h);
  return h;
}

void palm::load(const std::filesystem::path& f, std::string& s) {
  std::ifstream file;
  file.exceptions(std::ifstream::failbit | std::ifstream::badbit);
  file.open(f, std::ios_base::binary);
  std::size_t size = static_cast<std::size_t>(std::filesystem::file_size(f));
  s.resize(size, '\0');
  file.read(&s[0], size);
}
void palm::load(const std::filesystem::path& f, std::vector<uint8_t> b) {
  std::ifstream file(f, std::ios::binary);
  b.reserve(static_cast<std::size_t>(std::filesystem::file_size(f)));
  std::copy(std::istream_iterator<uint8_t>(file),
            std::istream_iterator<uint8_t>(), std::back_inserter(b));
}
