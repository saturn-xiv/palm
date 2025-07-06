#include <catch2/catch_test_macros.hpp>

#include "palm/s3.hpp"

#include <cstdlib>

#include <boost/algorithm/string/join.hpp>

TEST_CASE("minio client", "[minio]") {
  palm::Minio cli(std::getenv("MINIO_BASE_URL"),
                  std::getenv("MINIO_ACCESS_KEY"),
                  std::getenv("MINIO_SECRET_KEY"));

  {
    const auto buckets = cli.list_buckets();
    std::cout << boost::algorithm::join(buckets, ", ") << std::endl;
  }
}
