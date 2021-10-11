#define BOOST_TEST_MODULE cache
#include <boost/test/unit_test.hpp>

#include "palm/cache.hpp"

BOOST_AUTO_TEST_CASE(redis) {
  const auto key = "hi";
  const auto val = "hello";
  toml::table cfg;

  cfg.insert("pool-size", 5);
  cfg.insert("zone", "test");
  std::cout << cfg << std::endl;

  auto builder = std::make_shared<palm::redis::Config>();
  *builder = cfg;
  auto pool = std::make_shared<palm::redis::Pool>(builder);

  {
    toml::table it;
    it << *builder;
    std::cout << "new config: " << it << std::endl;
  }

  auto it = pool->get();
  palm::redis::PooledConnection(pool, it);

  it->palm::cache::Cache::set(key, val);

  for (int i = 1; i < 10; i++) {
    std::stringstream ss;
    ss << key << "/" << i;
    it->set(ss.str(), val, std::chrono::hours{i});
  }

  const auto tmp = it->get(key);
  BOOST_CHECK_EQUAL(val, tmp);

  for (const auto& it : it->keys()) {
    std::cout << it.first << "\t" << it.second << std::endl;
  }
}
