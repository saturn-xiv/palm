#include <catch2/catch_test_macros.hpp>

#include "palm/s3.hpp"

#include <cstdlib>

#include <boost/algorithm/string/join.hpp>

TEST_CASE("minio client", "[minio]") {
  spdlog::set_level(spdlog::level::debug);

  // palm::minio::Client cli(std::getenv("MINIO_BASE_URL"),
  //                         std::getenv("MINIO_ACCESS_KEY"),
  //                         std::getenv("MINIO_SECRET_KEY"));

  const auto host = std::getenv("MINIO_HOST");

  SECTION("config") {
    std::ifstream fs(std::format("{}.json", host));
    auto js = nlohmann::json::parse(fs);
    std::cout << js.dump(2) << std::endl;
    auto cfg = js.template get<palm::minio::Config>();
    std::cout << "parse url: " << cfg.url << "\t" << cfg.path << std::endl;
    REQUIRE(!cfg.url.empty());
    const auto url = boost::urls::parse_uri(cfg.url);
    REQUIRE(url.has_value());
    std::stringstream endpoint;
    {
      endpoint << url->scheme() << "://" << url->host();
      if (url->has_port()) {
        endpoint << ":" << url->port();
      }
    }
    std::cout << endpoint.str() << std::endl;
  }

  auto cli = palm::minio::Client::open(host);
  SECTION("list-buckets") {
    const auto buckets = cli->list_buckets();
    std::cout << boost::algorithm::join(buckets, ", ") << std::endl;
  }
  SECTION("upload-object") {
    const auto bucket = "testing";
    const auto file = std::getenv("MINIO_FILE_TO_UPLOAD");
    const auto object = palm::minio::Client::object(file);
    if (file != nullptr) {
      if (!cli->bucket_exists(bucket)) {
        cli->create_bucket(bucket, false, {1});
      }
      const auto ok = cli->upload(bucket, object, file);
      REQUIRE(ok);
    }
    const auto url = cli->get_presigned_object_url(bucket, object, file,
                                                   "application/octet-stream");
    REQUIRE(url.has_value());
    std::cout << url.value() << std::endl;
  }
}
