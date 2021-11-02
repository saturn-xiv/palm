#include "palm/fig.hpp"

#include <boost/program_options.hpp>

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/health_check_service_interface.h>

#include <sodium.h>

void mount(httplib::Server& svr) {
  palm::forum::mount(svr);
  palm::nut::mount(svr);
}

int main(int ac, char** av) {
  if (sodium_init() < 0) {
    BOOST_LOG_TRIVIAL(error) << "failed to init sodium";
    return EXIT_FAILURE;
  }
  try {
    std::string config;

    boost::program_options::options_description generic("Generic options");
    generic.add_options()(
        "config,c",
        boost::program_options::value<std::string>(&config)->default_value(
            "config.toml"),
        "configruation file")("version,v", "print version string")(
        "help,h", "produce help message")("debug,d", "run in debug mode");

    boost::program_options::options_description generate("Generate operations");
    generate.add_options()("generate-nginx", "generate nginx.conf files")(
        "generate-systemd", "generate systemd service.conf files")(
        "generate-config", "generate a config.toml file")(
        "generate-db-migration", "generate a database migration")(
        "name", boost::program_options::value<std::string>())(
        "domain", boost::program_options::value<std::string>())(
        "ssl", "enable openssl certificates");

    boost::program_options::options_description database("Database operations");
    database.add_options()("db-migrate",
                           "migrate database to the latest version")(
        "db-rollback", "rollback database to the previous version")(
        "db-status", "show databse current versions");

    boost::program_options::options_description cache("Cache operations");
    cache.add_options()("cache-list", "show items in the cache")("cache-clear",
                                                                 "clear cache");

    boost::program_options::options_description servers("Servers");
    servers.add_options()("web", "start a HTTP web server")(
        "rpc", "start a gRPC server")("worker", "start a queue worker");

    boost::program_options::options_description usage;
    usage.add(generic).add(generate).add(database).add(cache).add(servers);

    boost::program_options::variables_map vm;
    boost::program_options::store(
        boost::program_options::parse_command_line(ac, av, usage), vm);
    boost::program_options::notify(vm);
    if (vm.count("help")) {
      std::cout << usage << std::endl;
      return EXIT_SUCCESS;
    }
    if (vm.count("version")) {
      std::cout << PALM_GIT_VERSION << "(" << PALM_BUILD_TIME << ")"
                << std::endl;
      return EXIT_SUCCESS;
    }
    if (vm.count("generate-config")) {
      palm::fig::app::generate_config(config);
      return EXIT_SUCCESS;
    }
    if (vm.count("generate-nginx")) {
      palm::fig::app::generate_nginx(vm["domain"].as<std::string>(),
                                     vm.count("ssl"));
      return EXIT_SUCCESS;
    }
    if (vm.count("generate-systemd")) {
      palm::fig::app::generate_config(vm["domain"].as<std::string>());
      return EXIT_SUCCESS;
    }
    if (vm.count("generate-db-migration")) {
      palm::fig::app::generate_db_migration(vm["name"].as<std::string>());
      return EXIT_SUCCESS;
    }
    if (vm.count("db-migrate")) {
      palm::fig::app::db_migrate();
      return EXIT_SUCCESS;
    }
    if (vm.count("db-rollback")) {
      palm::fig::app::db_rollback();
      return EXIT_SUCCESS;
    }
    if (vm.count("db-status")) {
      palm::fig::app::db_status();
      return EXIT_SUCCESS;
    }
    if (vm.count("cache-list")) {
      palm::fig::app::cache_list();
      return EXIT_SUCCESS;
    }
    if (vm.count("cache-clear")) {
      palm::fig::app::cache_clear();
      return EXIT_SUCCESS;
    }
    if (vm.count("web")) {
      palm::fig::app::start_web();
      return EXIT_SUCCESS;
    }
    if (vm.count("rpc")) {
      palm::fig::app::start_rpc();
      return EXIT_SUCCESS;
    }
    if (vm.count("worker")) {
      palm::fig::app::start_worker();
      return EXIT_SUCCESS;
    }

    return EXIT_SUCCESS;

  } catch (std::exception& e) {
    BOOST_LOG_TRIVIAL(error) << e.what();
    return EXIT_FAILURE;
  }

  // BOOST_LOG_TRIVIAL(info) << "start " << PALM_GIT_VERSION << "("
  //                         << PALM_BUILD_TIME << ")";

  // httplib::Server svr;
  // mount(svr);
  // // TODO svr.stop();
  // svr.listen("127.0.0.1", 1234);
}
