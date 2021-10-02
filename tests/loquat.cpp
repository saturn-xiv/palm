#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include "palm/loquat.hpp"

TEST_CASE("print value", "[loquat]") {
  std::vector<palm::loquat::Value> items;
  items.push_back(true);
  items.push_back(1);
  items.push_back(2.2f);
  items.push_back("str");
  items.push_back(std::vector<int>({11, 22, 33}));
  items.push_back(std::vector<float>({11.1, 22.2, 33.2}));
  items.push_back(std::vector<std::string>({"s1", "s2", "s3"}));
  for (const auto& it : items) {
    std::cout << std::visit(palm::loquat::value2string(), it) << std::endl;
    {
      nlohmann::json js = it;
      std::cout << js << std::endl;
    }
  }
}

TEST_CASE("print env", "[loquat]") {
  const auto node = toml::parse_file("staging/hosts/all.toml");
  const auto env = palm::loquat::load(node);
  for (const auto& [k, v] : env) {
    std::cout << k << "=" << std::visit(palm::loquat::value2string(), v)
              << std::endl;
  }
}

TEST_CASE("load tasks", "[loquat]") {
  palm::set_logger(true);
  Poco::Logger& logger = Poco::Logger::root();
  auto lstr = std::make_shared<Poco::LogStream>(logger);

  palm::loquat::Inventory inventory("staging", lstr);
  std::cout << inventory << std::endl;
  palm::loquat::Job job("ping", lstr);
  std::cout << job << std::endl;
}
