#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

TEST_CASE("Demo", "[f]") { REQUIRE(2 * 2 == 4); }

// #define BOOST_TEST_MODULE orm
// #include <boost/test/included/unit_test.hpp>

// // #include "palm/orm.hpp"

// BOOST_AUTO_TEST_CASE(queries) {
//   // palm::orm::Schema::load_queries("db");
//   // for (const auto& it : {"insert", "delete", "set-run-at"}) {
//   //   std::stringstream ss;
//   //   ss << "schema_migrations." << it;
//   //   const auto key = ss.str();
//   //   std::cout << key << " = " << palm::orm::Schema::query(key) <<
//   std::endl;
//   // }
// }

// BOOST_AUTO_TEST_CASE(sqlite3) {
//   // palm::sqlite3::init();
//   // auto sql = palm::sqlite3::open("demo");

//   // std::tm now;
//   // (*sql) << "SELECT CURRENT_TIMESTAMP", soci::into(now);

//   // std::cout << std::asctime(&now) << std::endl;
// }

// BOOST_AUTO_TEST_CASE(postgresql) {
//   // palm::postgresql::init();

//   // toml::table cfg;
//   // cfg.insert("name", "demo");
//   // cfg.insert("pool-size", 5);

//   // auto builder = std::make_shared<palm::postgresql::Config>();
//   // *builder = cfg;
//   // auto pool = std::make_shared<palm::postgresql::Pool>(builder);

//   // auto it = pool->get();
//   // palm::postgresql::PooledConnection(pool, it);

//   // std::tm now;
//   // (*it) << "SELECT CURRENT_TIMESTAMP", soci::into(now);

//   // pool->heartbeat([](std::shared_ptr<soci::session> it) -> bool {
//   //   (*it) << "SELECT 1";
//   //   return true;
//   // });

//   // std::cout << std::asctime(&now) << std::endl;
// }

// BOOST_AUTO_TEST_CASE(mysql) {
//   // palm::mysql::init();
//   // toml::table cfg;
//   // cfg.insert("name", "demo");
//   // cfg.insert("pool-size", 5);

//   // auto builder = std::make_shared<palm::mysql::Config>();
//   // *builder = cfg;
//   // auto pool = std::make_shared<palm::mysql::Pool>(builder);

//   // auto it = pool->get();
//   // palm::mysql::PooledConnection(pool, it);

//   // std::tm now;
//   // (*it) << "SELECT CURRENT_TIMESTAMP", soci::into(now);

//   // std::cout << std::asctime(&now) << std::endl;
// }
