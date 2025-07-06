#include <catch2/catch_test_macros.hpp>

#include "palm/search.hpp"

#include <iostream>

namespace palm_testing {
struct EchoMessage {
  int id;
  std::string line;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(EchoMessage, id, line);
};
}  // namespace palm_testing

TEST_CASE("by restful", "[opensearch]") {
  palm::opensearch::Client client;
  {
    const auto val = client.cluster_health();
    REQUIRE(val.has_value());
    std::cout << "OpenSearch: " << val->cluster_name << "(" << val->status
              << ")" << std::endl;
  }
  {
    std::cout << "echo index: "
              << client.index_name<palm_testing::EchoMessage>() << std::endl;

    if (client.index_exists<palm_testing::EchoMessage>()) {
      client.delete_index<palm_testing::EchoMessage>();
    }

    {
      nlohmann::json props;

      {
        nlohmann::json it;
        it["type"] = "integer";
        props["id"] = it;
      }
      {
        nlohmann::json it;
        it["type"] = "text";
        props["line"] = it;
      }
      const auto val =
          client.create_index<palm_testing::EchoMessage>(2, 1, props);
      REQUIRE(val.has_value());
    }
  }

  for (int i = 0; i < 10; i++) {
    palm_testing::EchoMessage it = {.id = i + 100,
                                    .line = std::format("hello, palm({})!", i)};
    const auto val = client.index_document(it);
    REQUIRE(val.has_value());
  }
  std::this_thread::sleep_for(std::chrono::seconds(5));
  {
    const auto val = client.count<palm_testing::EchoMessage>();
    REQUIRE(val.has_value());
    REQUIRE(val.value() > 0);
  }
}
