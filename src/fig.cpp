#include "palm/fig.hpp"
#include "palm/cbeta.hpp"
#include "palm/cms.hpp"
#include "palm/cscd.hpp"
#include "palm/forum.hpp"
#include "palm/mall.hpp"
#include "palm/nut.hpp"
#include "palm/ops-email.hpp"
#include "palm/ops-metrics.hpp"
#include "palm/ops-vpn.hpp"
#include "palm/school.hpp"
#include "palm/version.hpp"

#include <iostream>

#include <boost/log/trivial.hpp>
#include <boost/program_options.hpp>
#include <boost/property_tree/ini_parser.hpp>

palm::fig::Application::Application(int argc, char** argv) {
  std::filesystem::path config_file;

  boost::program_options::options_description generic("Generic options");
  generic.add_options()("version,v", "print version string")(
      "help,h", "produce help message")("debug,d", "run on debug mode")(
      "config,c",
      boost::program_options::value<std::filesystem::path>(&config_file)
          ->default_value("config.ini"),
      "config file path");

  boost::program_options::options_description db("PostgreSql");
  db.add_options()("db-migrate", "runs all pending migrations")(
      "db-rollback", "reverts the latest run migration")(
      "db-list", "lists all available migrations");

  boost::program_options::options_description cache("Redis");
  cache.add_options()("cache-list", "lists all available cache items")(
      "cache-clear", "clear cache items");

  boost::program_options::options_description services("Services");
  services.add_options()("rpc", "start rpc server")("web", "start web server")(
      "worker", "start worker cosumer");

  boost::program_options::options_description args;
  args.add(generic).add(db).add(cache).add(services);

  boost::program_options::variables_map vm;
  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, args), vm);
  boost::program_options::notify(vm);

  if (vm.count("help")) {
    std::cout << args << std::endl;
    return;
  }
  if (vm.count("version")) {
    std::cout << PALM_GIT_VERSION << "(" << PALM_BUILD_TIME << ")" << std::endl;
    return;
  }

  palm::nut::init_logger(vm.count("debug"));
  BOOST_LOG_TRIVIAL(debug) << "load config file from " << config_file;
  boost::property_tree::ptree config;
  boost::property_tree::read_ini(config_file, config);

  BOOST_LOG_TRIVIAL(debug) << "connect postgresql server";
  const palm::PostgreSQL pgsql_c(config);
  auto pgsql = pgsql_c.open();
  std::string pgsql_schema_dir =
      config.get<std::string>("postgresql.schema-dir");
  const auto queries = palm::orm::queries(pgsql_schema_dir);
  BOOST_LOG_TRIVIAL(debug) << "connect redis server";
  const palm::Redis redis_c(config);
  auto redis = redis_c.open();
  BOOST_LOG_TRIVIAL(debug) << "connect rabbitmq server";
  // TODO
  BOOST_LOG_TRIVIAL(debug) << "connect elasticsearch server";
  // TODO
  {
    BOOST_LOG_TRIVIAL(debug) << "init postgresql migrations";
    Poco::Data::Session db(pgsql->get());
    palm::orm::Schema schema(db, queries);

    if (vm.count("db-migrate")) {
      schema.load(pgsql_schema_dir);
      schema.migrate();
      return;
    }
    if (vm.count("db-rollback")) {
      schema.load(pgsql_schema_dir);
      schema.rollback();
      return;
    }
    if (vm.count("db-list")) {
      schema.load(pgsql_schema_dir);
      std::cout << schema << std::endl;
      return;
    }
  }
  {
    if (vm.count("cache-list")) {
      Poco::Redis::PooledConnection db(*redis);
      palm::Cache ch(db, redis_c.ns());
      const auto flags = std::cout.flags();
      std::cout << std::setiosflags(std::ios::left);
      std::cout << std::setw(12) << "TTL"
                << " "
                << "KEY" << std::endl;

      for (const auto& [key, ttl] : ch.keys()) {
        std::cout << std::setw(12) << ttl << " " << key << std::endl;
      }
      return;
    }
    if (vm.count("cache-clear")) {
      Poco::Redis::PooledConnection db(*redis);
      palm::Cache ch(db, redis_c.ns());
      ch.clear();
      return;
    }
  }

  if (vm.count("rpc")) {
    this->rpc(config);
    return;
  }
  if (vm.count("web")) {
    this->web(config);
    return;
  }
  if (vm.count("worker")) {
    this->worker(config);
    return;
  }
  std::cout << args << std::endl;
}

void palm::fig::Application::rpc(
    const boost::property_tree::ptree& config) const {
  uint16_t port = config.get("rpc.port", 9999);
  std::stringstream address;
  address << "0.0.0.0:" << port;
  // TODO
  BOOST_LOG_TRIVIAL(info) << "server listening on " << address.str();
}

void palm::fig::Application::web(
    const boost::property_tree::ptree& tree) const {
  // TODO
}
void palm::fig::Application::worker(
    const boost::property_tree::ptree& tree) const {
  // TODO
}
