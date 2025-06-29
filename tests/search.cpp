#define BOOST_TEST_MODULE search
#include <boost/test/included/unit_test.hpp>

#include "basil/search.hpp"

namespace basil_testing {
struct EchoMessage {
  int id;
  std::string line;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(EchoMessage, id, line);
};
}  // namespace basil_testing

BOOST_AUTO_TEST_CASE(opensearch) {
  basil::opensearch::Client client;
  {
    const auto val = client.cluster_health();
    BOOST_REQUIRE(val.has_value());
    std::cout << "OpenSearch: " << val->cluster_name << "(" << val->status
              << ")" << std::endl;
  }
  {
    std::cout << "echo index: "
              << client.index_name<basil_testing::EchoMessage>() << std::endl;

    if (client.index_exists<basil_testing::EchoMessage>()) {
      client.delete_index<basil_testing::EchoMessage>();
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
          client.create_index<basil_testing::EchoMessage>(2, 1, props);
      BOOST_REQUIRE(val.has_value());
    }
  }

  for (int i = 0; i < 10; i++) {
    basil_testing::EchoMessage it = {
        .id = i + 100, .line = std::format("hello, basil({})!", i)};
    const auto val = client.index_document(it);
    BOOST_REQUIRE(val.has_value());
  }
  std::this_thread::sleep_for(std::chrono::seconds(5));
  {
    const auto val = client.count<basil_testing::EchoMessage>();
    BOOST_REQUIRE(val.has_value());
    BOOST_REQUIRE_GT(val.value(), 0);
  }
}
