#include <cstdlib>

// #include "palm/fig.hpp"

// #include <grpcpp/ext/proto_server_reflection_plugin.h>
// #include <grpcpp/grpcpp.h>
// #include <grpcpp/health_check_service_interface.h>
// #include <boost/log/core.hpp>
// #include <boost/log/expressions.hpp>
// #include <boost/log/trivial.hpp>
// #include <boost/program_options.hpp>

int main(int argc, char* argv[]) {
  // boost::program_options::options_description generic("Generic options");
  // std::filesystem::path config;
  // generic.add_options()("version,v", "print version string")(
  //     "help,h", "produce help message")("debug,d", "enable debug mode")(
  //     "config,c",
  //     boost::program_options::value<std::filesystem::path>(&config)
  //         ->default_value("config.toml"),
  //     "set configuration file");

  // boost::program_options::options_description generate("File grnerate
  // options"); generate.add_options()("generate-config", "generate
  // config.toml")(
  //     "generate-systemd", "generate systemd service files")(
  //     "generate-migration", "generate a database migration")(
  //     "generate-nginx", "generate a nginx.conf")("ssl", "enable https")(
  //     "name", boost::program_options::value<std::string>(), "name")(
  //     "domain", boost::program_options::value<std::string>(), "domain name");

  // boost::program_options::options_description db("Database options");
  // std::filesystem::path migration_dir;
  // db.add_options()("db-setup", "creates the database & user")(
  //     "db-migrate", R"RAW(migrate the database to the latest version)RAW")(
  //     "db-rollback", R"RAW(rolls the schema back to the previous
  //     version)RAW")( "db-status", R"RAW(retrieves the current schema version
  //     number)RAW")( "migration-dir",
  //     boost::program_options::value<std::filesystem::path>(&migration_dir)
  //         ->default_value(std::filesystem::path("db") / "postgresql"),
  //     "the location of your migration directory.");

  // boost::program_options::options_description cache("Cache options");
  // cache.add_options()("cache-list", "list all items in cache")(
  //     "cache-purge", "remove all cache items");

  // boost::program_options::options_description job("Job options");
  // job.add_options()("rpc", "start a rpc process")("web", "start a web
  // process")(
  //     "worker", "start a worker process");

  // boost::program_options::options_description desc;
  // desc.add(generic).add(generate).add(db).add(cache).add(job);

  // boost::program_options::variables_map vm;
  // boost::program_options::store(
  //     boost::program_options::parse_command_line(argc, argv, desc), vm);
  // boost::program_options::notify(vm);

  // if (vm.count("help")) {
  //   std::cout << desc << std::endl;
  //   return EXIT_SUCCESS;
  // }
  // if (vm.count("version")) {
  //   std::cout << PALM_GIT_VERSION << "(" << PALM_BUILD_TIME << ")" <<
  //   std::endl; return EXIT_SUCCESS;
  // }

  // boost::log::core::get()->set_filter(boost::log::trivial::severity >=
  //                                     (vm.count("debug")
  //                                          ? boost::log::trivial::debug
  //                                          : boost::log::trivial::info));
  // BOOST_LOG_TRIVIAL(info) << "start " << PALM_GIT_VERSION << "("
  //                         << PALM_BUILD_TIME << ")";
  // BOOST_LOG_TRIVIAL(debug) << "use debug mode";
  // if (vm.count("generate-config")) {
  //   palm::fig::Config cfg;
  //   toml::table node;
  //   node << cfg;

  //   const auto fn = std::filesystem::temp_directory_path() / config;
  //   BOOST_LOG_TRIVIAL(info) << "write to file " << fn.string();
  //   if (std::filesystem::exists(fn)) {
  //     throw std::invalid_argument("file already exists");
  //   }
  //   std::ofstream fd;
  //   fd.open(fn);
  //   fd << node;
  //   fd.close();
  //   return EXIT_SUCCESS;
  // }
  // if (vm.count("generate-systemd")) {
  //   const auto domain = vm["domain"].as<std::string>();
  //   palm::fig::Systemd cfg(domain, PALM_PROJECT_DESCRIPTION);
  //   cfg.rpc();
  //   cfg.web();
  //   cfg.worker();
  //   return EXIT_SUCCESS;
  // }
  // if (vm.count("generate-migration")) {
  //   const auto name = vm["name"].as<std::string>();
  //   palm::orm::Migration::generate(migration_dir, name);
  //   return EXIT_SUCCESS;
  // }

  // auto cfg = std::make_shared<palm::fig::Config>();
  // {
  //   const auto file = config.string();
  //   BOOST_LOG_TRIVIAL(info) << "load config from " << file;
  //   const auto node = toml::parse_file(file);
  //   *cfg = node;
  // }
  // if (vm.count("generate-nginx")) {
  //   const auto domain = vm["domain"].as<std::string>();
  //   const auto ssl = vm.count("ssl");

  //   cfg->nginx_conf(domain, ssl);
  //   return EXIT_SUCCESS;
  // }
  // {
  //   palm::orm::Schema::load_queries(migration_dir);
  //   palm::sqlite3::init();
  //   palm::mysql::init();
  //   palm::postgresql::init();
  // }
  // {
  //   auto db = cfg->postgresql.build();
  //   palm::orm::Schema schema(db, migration_dir);
  //   if (vm.count("db-migrate")) {
  //     schema.migrate();
  //     return EXIT_SUCCESS;
  //   }
  //   if (vm.count("db-rollback")) {
  //     schema.rollback();
  //     return EXIT_SUCCESS;
  //   }
  //   if (vm.count("db-status")) {
  //     schema.status();
  //     return EXIT_SUCCESS;
  //   }
  // }

  // auto app = std::make_shared<palm::fig::Application>(cfg);
  // if (vm.count("web")) {
  //   app->web();
  //   return EXIT_SUCCESS;
  // }
  // if (vm.count("rpc")) {
  //   app->rpc();
  //   return EXIT_SUCCESS;
  // }
  // if (vm.count("worker")) {
  //   app->worker();
  //   return EXIT_SUCCESS;
  // }
  return EXIT_SUCCESS;
}
