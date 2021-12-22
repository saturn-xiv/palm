#include "palm/fig.hpp"
#include "palm/auth.hpp"
#include "palm/version.hpp"

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
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

  palm::init_logger(vm.count("debug"));
  BOOST_LOG_TRIVIAL(info) << "load config file from " << config_file;
  boost::property_tree::ptree config;
  boost::property_tree::read_ini(config_file, config);

  if (vm.count("rpc")) {
    this->rpc(config);
    return;
  }
}

void palm::fig::Application::rpc(const boost::property_tree::ptree& config) {
  std::stringstream address_s;
  {
    uint16_t port = config.get("rpc.port", 8086);
    address_s << "0.0.0.0:" << port;
  }

  const std::string address = address_s.str();
  palm::auth::UserService auth_user;

  grpc::EnableDefaultHealthCheckService(true);
  grpc::reflection::InitProtoReflectionServerBuilderPlugin();
  grpc::ServerBuilder builder;

  builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  builder.RegisterService(&auth_user);

  std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
  BOOST_LOG_TRIVIAL(info) << "server listening on " << address;

  server->Wait();
}

void palm::fig::Application::web(const boost::property_tree::ptree& config) {
  // TODO
}
void palm::fig::Application::worker(const boost::property_tree::ptree& config) {
  // TODO
}
