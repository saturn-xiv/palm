#include <catch2/benchmark/catch_benchmark.hpp>
#include <catch2/catch_test_macros.hpp>

#include "palm/cache.hpp"
#include "portal.pb.h"

#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>

TEST_CASE("redis client", "[cache]") {
  palm::redis::Config config("127.0.0.1", 6371);
  auto client = config.open();

  SECTION("raw string message") {
    const std::string key = "hi.raw-string";
    const std::string value = "Hello, Palm!";

    REQUIRE(client->set(key, value,
                        std::chrono::duration_cast<std::chrono::seconds>(
                            std::chrono::hours(2))));
    {
      const auto tmp = client->get(key);
      REQUIRE(tmp.has_value());
      REQUIRE(tmp.value() == value);
    }
  }
  SECTION("protobuf message") {
    const auto id = 123;
    const auto lang = "en-US";
    const auto code = "hi";
    const auto message = "Hello, Palm!";
    const std::string key = "hi.protobuf";

    const auto updated_at = google::protobuf::util::TimeUtil::GetCurrentTime();
    std::cout << "Current Time: "
              << google::protobuf::util::TimeUtil::ToString(updated_at)
              << std::endl;

    palm::portal::v1::LocaleIndexResponse_Item value;
    value.set_id(id);
    value.set_lang(lang);
    value.set_code(code);
    value.set_message(message);

    {
      auto it = value.mutable_updated_at();
      it->MergeFrom(updated_at);
    }

    REQUIRE(client->set(key, &value,
                        std::chrono::duration_cast<std::chrono::seconds>(
                            std::chrono::hours(2))));
    {
      palm::portal::v1::LocaleIndexResponse_Item tmp;
      REQUIRE(client->get(key, &tmp));
      REQUIRE(id == tmp.id());
      REQUIRE(lang == tmp.lang());
      REQUIRE(code == tmp.code());
      REQUIRE(message == tmp.message());
      {
        REQUIRE(tmp.updated_at().seconds() > 0);
        REQUIRE(tmp.updated_at().seconds() == updated_at.seconds());
        REQUIRE(tmp.updated_at().nanos() == updated_at.nanos());
      }

      {
        std::string buf;
        const auto status =
            google::protobuf::util::MessageToJsonString(tmp, &buf);
        REQUIRE(status.ok());
        std::cout << "Message: " << buf << std::endl;
      }
    }
  }
}
