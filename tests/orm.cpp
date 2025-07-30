#include <catch2/catch_test_macros.hpp>

#include "palm/orm.hpp"
#include "palm/theme.hpp"

#include <iostream>

#include <boost/date_time/posix_time/posix_time.hpp>

#include <google/protobuf/util/time_util.h>

TEST_CASE("boost ptime", "[boost]") {
  boost::posix_time::ptime max(boost::posix_time::max_date_time);
  boost::posix_time::ptime min(boost::posix_time::min_date_time);
  boost::posix_time::ptime utc_s =
      boost::posix_time::second_clock::universal_time();
  boost::posix_time::ptime utc_ms =
      boost::posix_time::microsec_clock::universal_time();

  std::cout << "boost(max): " << boost::posix_time::to_iso_extended_string(max)
            << std::endl;
  std::cout << "boost(min): " << boost::posix_time::to_iso_extended_string(min)
            << std::endl;
  std::cout << "boost(utc second): "
            << boost::posix_time::to_iso_extended_string(utc_s) << std::endl;
  std::cout << "boost(utc microsecond): "
            << boost::posix_time::to_iso_extended_string(utc_ms) << std::endl;

  {
    const std::string s = "2025-07-13 14:32:07.005979+00";
    google::protobuf::Arena arena;
    google::protobuf::Timestamp* it =
        google::protobuf::Arena::Create<google::protobuf::Timestamp>(&arena);

    palm::str2ts(s, it);
    REQUIRE(it->seconds() == 1752417127);
    REQUIRE(it->nanos() == 5979 * 1000);
    const auto j = palm::to_json(*it);
    REQUIRE(j.has_value());
    std::cout << "google timestamp: " << "(" << it->seconds() << ","
              << it->nanos() << ") " << j.value() << std::endl;
  }
}

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
  SECTION("timestamp") {
    soci::session db(*pool);
    {
      std::string now;
      db << "SELECT CURRENT_TIMESTAMP", soci::into(now);
      std::cout << "timestamp(str): " << now << std::endl;

      {
        google::protobuf::Arena arena;
        google::protobuf::Timestamp* it =
            google::protobuf::Arena::Create<google::protobuf::Timestamp>(
                &arena);

        palm::str2ts(now, it);
        // REQUIRE(google::protobuf::util::TimeUtil::FromString(now, it));
        std::cout << "timestamp(google): " << it->seconds() << ", "
                  << it->nanos() << std::endl;
      }
    }
    {
      std::tm now;
      db << "SELECT CURRENT_TIMESTAMP", soci::into(now);
      std::cout << "timestamp(tm): " << std::asctime(&now) << std::endl;
      {
        const time_t seconds = std::mktime(&now);
        std::cout << std::ctime(&seconds) << std::endl;
      }
    }
    {
      boost::gregorian::date today;
      db << "SELECT CURRENT_DATE", soci::into(today);
      std::cout << "date(boost): " << today << std::endl;
    }
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
