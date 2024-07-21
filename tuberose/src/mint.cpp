#include "palm/mint.hpp"
#include "palm/version.hpp"

#include <argparse/argparse.hpp>

void palm::mint::PostgreSql::migrate(const std::string& id,
                                     const std::string& name,
                                     const std::string& sql) {
  // TODO
}
void palm::mint::PostgreSql::rollback(const std::string& id,
                                      const std::string& name,
                                      const std::string& sql) {
  // TODO
}
std::vector<palm::mint::Migration> palm::mint::PostgreSql::status() {
  std::vector<palm::mint::Migration> items;
  // TODO
  return items;
}
void palm::mint::PostgreSql::create(){
    // TODO
};
void palm::mint::PostgreSql::drop(){
    // TODO
};
void palm::mint::MySql::migrate(const std::string& id, const std::string& name,
                                const std::string& sql) {
  // TODO
}
void palm::mint::MySql::rollback(const std::string& id, const std::string& name,
                                 const std::string& sql) {
  // TODO
}
void palm::mint::MySql::create(){
    // TODO
};
void palm::mint::MySql::drop(){
    // TODO
};
std::vector<palm::mint::Migration> palm::mint::MySql::status() {
  std::vector<palm::mint::Migration> items;
  // TODO
  return items;
}
void palm::mint::Sqlite3::migrate(const std::string& id,
                                  const std::string& name,
                                  const std::string& sql) {
  // TODO
}
void palm::mint::Sqlite3::rollback(const std::string& id,
                                   const std::string& name,
                                   const std::string& sql) {
  // TODO
}
void palm::mint::Sqlite3::create(){
    // TODO
};
void palm::mint::Sqlite3::drop(){
    // TODO
};
std::vector<palm::mint::Migration> palm::mint::Sqlite3::status() {
  std::vector<palm::mint::Migration> items;
  // TODO
  return items;
}

palm::mint::Schema::Schema(const std::filesystem::path& config,
                           const std::filesystem::path& migrations) {
  // TODO
}
void palm::mint::Application::generate(const std::string& name) {
  spdlog::info("generate migration folder {}", name);
  // TODO
}
void palm::mint::Application::migrate() {
  spdlog::info("apply all migrations");
  // TODO
}
void palm::mint::Application::migrate(const size_t steps) {
  spdlog::info("apply {} migrations");
  // TODO
}
void palm::mint::Application::migrate(const std::string& id) {
  spdlog::info("apply migration {}");
  // TODO
}
void palm::mint::Application::rollback(const size_t steps) {
  spdlog::debug("revert recent {} migrations", steps);
  // TODO
}
void palm::mint::Application::rollback(const std::string& id) {
  spdlog::warn("revert migration {}", id);
  // TODO
}
void palm::mint::Application::reset() {
  spdlog::warn("try to reset the database");
  // TODO
}
std::vector<palm::mint::Migration> palm::mint::Application::load(
    const std::filesystem::path& root) {
  std::vector<palm::mint::Migration> items;
  // TODO
  return items;
}

palm::mint::Application::Application(int argc, char** argv) {
  const std::string version = palm::GIT_VERSION + "(" + palm::BUILD_TIME + ")";
  argparse::ArgumentParser program(argv[0], version);
  program.add_description(
      R"(An open-source database-independent library for tracking, managing and applying database schema changes.)");
  program.add_epilog(palm::PROJECT_HOMEPAGE);
  program.add_argument("-d", "--debug")
      .default_value(false)
      .help("run on debug mode")
      .implicit_value(true);
  program.add_argument("-f", "--folder")
      .default_value("migrations")
      .help("migration files folder");
  program.add_argument("-c", "--config")
      .default_value("config.toml")
      .help("database connection configuration file");

  argparse::ArgumentParser generate_command("generate");
  { generate_command.add_argument("-n", "--name").required(); }

  argparse::ArgumentParser migrate_command("migrate");
  {
    migrate_command.add_argument("-s", "--steps").scan<'i', size_t>();
    migrate_command.add_argument("-i", "--id");
  }
  argparse::ArgumentParser rollback_command("rollback");
  {
    rollback_command.add_argument("-s", "--steps")
        .default_value(1)
        .scan<'i', size_t>();
    rollback_command.add_argument("-i", "--id");
  }
  argparse::ArgumentParser reset_command("reset");
  argparse::ArgumentParser status_command("status");
  program.add_subparser(generate_command);
  program.add_subparser(migrate_command);
  program.add_subparser(rollback_command);
  program.add_subparser(reset_command);
  program.add_subparser(status_command);

  try {
    program.parse_args(argc, argv);
  } catch (const std::runtime_error& err) {
    spdlog::error("{}", err.what());
    std::exit(1);
  }
  {
    spdlog::set_level(program.get<bool>("--debug") ? spdlog::level::debug
                                                   : spdlog::level::info);
    spdlog::debug("run on debug mode {}", version);
  }

  const std::string config = rpc_command.get<std::string>("--config");
  const std::string folder = rpc_command.get<std::string>("--folder");
  if (program.is_subcommand_used("generate")) {
    const std::string name = rpc_command.get<std::string>("--name");

    return;
  }
  if (program.is_subcommand_used("reset")) {
    this->reset();
    return;
  }
}

// #include <filesystem>

// #include <boost/log/trivial.hpp>
// #include <boost/program_options.hpp>

// palm::mint::Application::Application(int argc, char** argv) {
//   std::filesystem::path config_file;

//   boost::program_options::options_description generic("Generic options");
//   generic.add_options()("version,v", "print version string")(
//       "help,h", "produce help message")("debug,d", "run on debug mode")(
//       "config,c",
//       boost::program_options::value<std::filesystem::path>(&config_file)
//           ->default_value("config.ini"),
//       "config file path");

//   boost::program_options::options_description crawlers("Crawlers");
//   crawlers.add_options()("journal",
//                          boost::program_options::value<std::string>(),
//                          "listen systemd journal by unit-name")(
//       "nginx-access", boost::program_options::value<std::filesystem::path>(),
//       "listen nginx.access.log")(
//       "nginx-error", boost::program_options::value<std::filesystem::path>(),
//       "listen nginx.error.log")("heartbeat", "sent a heartbeat package")(
//       "snmp", "get/walk snmp agents");

//   boost::program_options::options_description args;
//   args.add(generic).add(crawlers);

//   boost::program_options::variables_map vm;
//   boost::program_options::store(
//       boost::program_options::parse_command_line(argc, argv, args), vm);
//   boost::program_options::notify(vm);

//   if (vm.count("help")) {
//     std::cout << args << std::endl;
//     return;
//   }
//   if (vm.count("version")) {
//     std::cout << palm::GIT_VERSION << "(" << palm::BUILD_TIME << ")"
//               << std::endl;
//     return;
//   }

//   palm::init_logger(vm.count("debug"));
//   BOOST_LOG_TRIVIAL(debug) << "load config file from " << config_file;
//   boost::property_tree::ptree config;
//   boost::property_tree::read_ini(config_file, config);

//   std::stringstream address;
//   {
//     const std::string host = config.get("rpc.host", "localhost");
//     const uint16_t port = config.get("rpc.port", 9090);
//     address << "0.0.0.0:" << port;
//   }
//   // TODO
//   std::cout << args << std::endl;
// }

// void palm::mint::Application::nginx(const std::string& address) {
//   if (palm::is_stopped()) {
//     return;
//   }
//   BOOST_LOG_TRIVIAL(info) << "connect to " << address;
//   palm::ops::metrics::ReportClient cli(
//       grpc::CreateChannel(address, grpc::InsecureChannelCredentials()));
//   // TODO
//   cli.nginx_error("bla bla bla");
// }

// void palm::mint::Application::journal(const std::string& host) {
//   if (palm::is_stopped()) {
//     return;
//   }
// }

// void palm::mint::Application::heartbeat(const std::string& host) {
//   if (palm::is_stopped()) {
//     return;
//   }
// }

// void palm::mint::Application::snmp(const std::string& host,
//                                    const boost::property_tree::ptree& config)
//                                    {
//   if (palm::is_stopped()) {
//     return;
//   }
// }
