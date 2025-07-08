#include <catch2/catch_test_macros.hpp>

#include "palm/orm.hpp"

#include <iostream>

TEST_CASE("by soci", "[postgresql]") {
  spdlog::set_level(spdlog::level::debug);

  palm::PostgreSql config("127.0.0.1", 5432, "www", "change-me", "lavender");
  auto pool = config.open();
  SECTION("version") {
    soci::session db(*pool);
    std::string version;
    db << "SELECT VERSION()", soci::into(version);
    std::cout << version << std::endl;
  }
}

TEST_CASE("by soci", "[mysql]") {}

TEST_CASE("by soci", "[sqlite3]") {
  spdlog::set_level(spdlog::level::debug);
  palm::Sqlite3 config("db.sqlite3");

  auto db = config.open();
  SECTION("version") {
    std::string version;
    *db << "SELECT SQLITE_VERSION()", soci::into(version);
    std::cout << "Sqlite3 Version: " << version << std::endl;
  }
}

TEST_CASE("pagination", "[postgresql,mysql,sqlite3]") {
  {
    const auto [index, size] = palm::paginate(100, -1, -1);
    REQUIRE(index == 1);
    REQUIRE(size == 10);
  }
  {
    const auto [index, size] = palm::paginate(100, 0, 0);
    REQUIRE(index == 1);
    REQUIRE(size == 10);
  }
  {
    const auto [index, size] = palm::paginate(100, -1, (1 << 12) + 1);
    REQUIRE(index == 1);
    REQUIRE(size == (1 << 12));
  }
  {
    const auto [index, size] = palm::paginate(100, 2, 200);
    REQUIRE(index == 1);
    REQUIRE(size == 200);
  }
  {
    const auto [index, size] = palm::paginate(100, 2, 20);
    REQUIRE(index == 2);
    REQUIRE(size == 20);
  }
  {
    const auto [index, size] = palm::paginate(100, 20, 20);
    REQUIRE(index == 5);
    REQUIRE(size == 20);
  }
  {
    const auto [index, size] = palm::paginate(101, 20, 20);
    REQUIRE(index == 6);
    REQUIRE(size == 20);
  }
}
