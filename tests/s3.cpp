#define BOOST_TEST_MODULE s3
#include <boost/test/included/unit_test.hpp>

#include "basil/s3.hpp"

#include <cstdlib>

#include <boost/algorithm/string/join.hpp>

BOOST_AUTO_TEST_CASE(minio_) {
  basil::Minio cli(std::getenv("MINIO_BASE_URL"),
                   std::getenv("MINIO_ACCESS_KEY"),
                   std::getenv("MINIO_SECRET_KEY"));

  {
    const auto buckets = cli.list_buckets();
    std::cout << boost::algorithm::join(buckets, ", ") << std::endl;
  }
  BOOST_CHECK_EQUAL(1 + 1, 2);
}
