#include "tulip/accounting.hpp"
#include "tulip/application.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"

// TODO import iso4217
static void load_iso4217_by_list_one(const std::filesystem::path& file) {
  // TODO
}
// TODO import locales by yaml
static void load_locales_by_yaml(const std::filesystem::path& folder) {
  // TODO
}

// https://docs.opensearch.org/latest/mappings/
static void init_search_engine(
    std::shared_ptr<palm::opensearch::Config> search) {
  // TODO
  if (!search->index_exists<palm::cms::v1::IndexPageResponse_Item>()) {
    search->create_index<palm::cms::v1::IndexPageResponse_Item>(
        R"(
{
  "permalink": {
    "type": "text"
  },
  "title": {
    "type": "text"
  }, 
  "summary": {
    "type": "text"
  }, 
  "author": {
    "type": "text"
  }, 
  "body": {
    "type": "text"
  },
  "published_at": {
    "type": "date",
    "format": "yyyy/MM/dd"
  },
  "updated_at": {
    "type": "date",
    "format": "yyyy/MM/dd"
  }
}
)"_json);
  }
}

static void init_queue(std::shared_ptr<palm::rabbitmq::Client> queue) {
  // TODO
}

int tulip::Application::db_seeds(const std::string& config_file) const {
  struct Config {
    Config(const toml::table& config)
        : postgresql(*(config["postgresql"].as_table())),
          rabbitmq(*(config["rabbitmq"].as_table())),
          opensearch(*(config["opensearch"].as_table())) {}

    palm::PostgreSql postgresql;
    palm::rabbitmq::Config rabbitmq;
    palm::opensearch::Config opensearch;
    palm::grpc::Config daisy;
  };

  const auto config_tree = toml::parse_file(config_file);
  Config config(config_tree);

  auto db = config.postgresql.open();
  auto queue = config.rabbitmq.open();
  spdlog::debug("open opensearch {}", config.opensearch.url(""));
  auto search = std::make_shared<palm::opensearch::Config>(config.opensearch);

  init_search_engine(search);
  init_queue(queue);

  return EXIT_SUCCESS;
}
