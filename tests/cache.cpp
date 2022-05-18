#define BOOST_TEST_MODULE cache
#include <boost/test/unit_test.hpp>

#include "palm/cache.hpp"
#include "palm/crypto.hpp"

#include <iostream>

#include <boost/property_tree/ini_parser.hpp>

#include <Poco/Redis/Array.h>
#include <Poco/Redis/Command.h>

BOOST_AUTO_TEST_CASE(redis) {
  boost::property_tree::ptree tree;
  boost::property_tree::read_ini("config.ini", tree);
  palm::Redis redis(tree);
  auto pool = redis.open();

  const std::string key = palm::random::alphanumeric(8) + ".hello";
  const std::string val = "Hi, Palm!";
  {
    Poco::Redis::PooledConnection db(*pool);
    Poco::Redis::Command cmd = Poco::Redis::Command::set(key, val);
    std::string rst = ((Poco::Redis::Client::Ptr)db)->execute<std::string>(cmd);
    BOOST_TEST(rst == "OK");
  }
  {
    Poco::Redis::PooledConnection db(*pool);
    Poco::Redis::Array cmd;
    cmd << "GET" << key;
    Poco::Redis::BulkString rst =
        ((Poco::Redis::Client::Ptr)db)->execute<Poco::Redis::BulkString>(cmd);
    BOOST_TEST(rst.value() == val);
  }
  {
    Poco::Redis::PooledConnection db(*pool);
    palm::Cache ch(db, redis.ns());
    ch.set(key, val);
    {
      const auto tmp = ch.get(key);
      BOOST_TEST(tmp.value() == val);
    }
    {
      const auto info = ch.status();
      std::cout << info << std::endl;
    }
  }
}
