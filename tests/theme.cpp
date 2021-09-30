#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

TEST_CASE("Demo", "[f]") { REQUIRE(2 * 2 == 4); }

// #define BOOST_TEST_MODULE theme
// #include <boost/test/included/unit_test.hpp>

// BOOST_AUTO_TEST_CASE(http_get) { BOOST_CHECK_EQUAL(1 + 1, 2); }

// BOOST_AUTO_TEST_CASE(http_post) { BOOST_CHECK_EQUAL(1 + 1, 2); }

// BOOST_AUTO_TEST_CASE(https_get) { BOOST_CHECK_EQUAL(1 + 1, 2); }

// BOOST_AUTO_TEST_CASE(https_post) { BOOST_CHECK_EQUAL(1 + 1, 2); }

// BOOST_AUTO_TEST_CASE(html_server) { BOOST_CHECK_EQUAL(1 + 1, 2); }
