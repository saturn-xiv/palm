#include "bamboo/models.hpp"
#include "router.grpc.pb.h"

#include <sodium.h>

static const std::string gl_administrator_name = "administrator.name";
static const std::string gl_administrator_password = "administrator.password";

void bamboo::dao::host::save(soci::session& db, const std::string& mac,
                             const std::string& name, const std::string& ip,
                             const std::string& vendor) {
  int c;
  db << R"SQL(SELECT COUNT(*) FROM hosts WHERE "mac" = :mac)SQL",
      soci::use(mac, "mac"), soci::into(c);
  if (c > 0) {
    db << R"SQL(UPDATE hosts SET ip=:ip, name=:name, vendor=:vendor, version=version+1, updated_at=CURRENT_TIMESTAMP WHERE "mac" = :mac)SQL",
        soci::use(mac, "mac"), soci::use(ip, "ip"), soci::use(name, "name"),
        soci::use(vendor, "vendor");
  } else {
    db << R"SQL(INSERT INTO hosts(mac, ip, name, vendor, updated_at) VALUES(:mac, :ip, :name, :vendor, CURRENT_TIMESTAMP))SQL",
        soci::use(mac, "mac"), soci::use(ip, "ip"), soci::use(name, "name"),
        soci::use(vendor, "vendor");
  }
}
void bamboo::dao::administrator::save(soci::session& db,
                                      const std::string& name,
                                      const std::string& password) {
  char hashed_password[crypto_pwhash_STRBYTES];
  if (crypto_pwhash_str(hashed_password, password.c_str(), password.length(),
                        crypto_pwhash_OPSLIMIT_SENSITIVE,
                        crypto_pwhash_MEMLIMIT_SENSITIVE) != 0) {
    spdlog::error("failed to compute password");
    return;
  }
  std::vector<uint8_t> name_(name.begin(), name.end());
  std::vector<uint8_t> password_(std::begin(hashed_password),
                                 std::end(hashed_password));
  bamboo::dao::set(db, gl_administrator_name, name_);
  bamboo::dao::set(db, gl_administrator_password, password_);
}

bool bamboo::dao::administrator::auth(soci::session& db,
                                      const std::string& name,
                                      const std::string& password) {
  {
    const auto val = bamboo::dao::get(db, gl_administrator_name);
    if (!val) {
      return false;
    }
    const std::string str(val->begin(), val->end());
    if (str != name) {
      return false;
    }
  }
  const auto password_ = bamboo::dao::get(db, gl_administrator_password);

  if (!password_) {
    return false;
  }
  if (password_->size() != crypto_pwhash_STRBYTES) {
    return false;
  }
  return crypto_pwhash_str_verify((char*)password_->data(), password.c_str(),
                                  password.length()) == 0;
}

void bamboo::dao::set(soci::session& db, const std::string& key,
                      const std::vector<uint8_t>& value) {
  int c;
  db << R"SQL(SELECT COUNT(*) FROM settings WHERE "key" = :key)SQL",
      soci::use(key, "key"), soci::into(c);
  if (c > 0) {
    db << R"SQL(UPDATE settings SET value=:value, version=version+1, updated_at=CURRENT_TIMESTAMP WHERE "key" = :key)SQL",
        soci::use(key, "key"), soci::use(value, "value");
  } else {
    db << R"SQL(INSERT INTO settings("key", value, updated_at) VALUES(:key, :value, CURRENT_TIMESTAMP))SQL",
        soci::use(key, "key"), soci::use(value, "value");
  }
}

std::optional<std::vector<uint8_t>> bamboo::dao::get(soci::session& db,
                                                     const std::string& key) {
  std::string val;
  soci::indicator ind;
  db << R"SQL(SELECT value FROM settings WHERE "key" = :key)SQL",
      soci::use(key, "key"), soci::into(val, ind);
  if (ind != soci::i_ok) {
    return std::nullopt;
  }

  std::vector<uint8_t> buf(val.begin(), val.end());
  return buf;
}

void bamboo::dao::set(soci::session& db, const std::vector<uint8_t>& secret,
                      const std::string& key,
                      const std::vector<uint8_t>& value) {
  uint8_t nonce[crypto_secretbox_NONCEBYTES];
  std::vector<uint8_t> code(crypto_secretbox_MACBYTES + value.size());
  randombytes_buf(nonce, sizeof nonce);
  if (0 != crypto_secretbox_easy(code.data(), value.data(), value.size(), nonce,
                                 secret.data())) {
    spdlog::error("failed to encrypt message");
    return;
  }
  std::vector<uint8_t> buf;
  buf.insert(buf.end(), std::begin(nonce), std::end(nonce));
  buf.insert(buf.end(), std::begin(code), std::end(code));
  bamboo::dao::set(db, key, buf);
}

std::optional<std::vector<uint8_t>> bamboo::dao::get(
    soci::session& db, const std::vector<uint8_t>& secret,
    const std::string& key) {
  const auto buf = bamboo::dao::get(db, key);
  if (!buf) {
    return std::nullopt;
  }
  if (buf->size() <=
      (crypto_secretbox_NONCEBYTES + crypto_secretbox_MACBYTES)) {
    spdlog::error("invalid cipher text length {}", buf->size());
    return std::nullopt;
  }
  std::vector<uint8_t> nonce(buf->begin(),
                             buf->begin() + crypto_secretbox_NONCEBYTES);
  std::vector<uint8_t> code(buf->begin() + crypto_secretbox_NONCEBYTES + 1,
                            buf->end());
  std::vector<uint8_t> plain(
      buf->size() - (crypto_secretbox_NONCEBYTES + crypto_secretbox_MACBYTES));
  if (crypto_secretbox_open_easy(plain.data(), code.data(), code.size(),
                                 nonce.data(), secret.data()) != 0) {
    spdlog::error("failed to decrypt message");
    return std::nullopt;
  }
  return plain;
}
