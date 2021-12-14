#include "palm/fig.hpp"
#include "palm/version.hpp"

#include <iostream>

#include <Poco/Util/HelpFormatter.h>
#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <boost/log/trivial.hpp>

void palm::fig::Rpc::defineOptions(Poco::Util::OptionSet& options) {}
void palm::fig::Rpc::initialize(Poco::Util::Application& app) {}

void palm::fig::Web::defineOptions(Poco::Util::OptionSet& options) {}
void palm::fig::Web::initialize(Poco::Util::Application& app) {}

void palm::fig::Worker::defineOptions(Poco::Util::OptionSet& options) {}
void palm::fig::Worker::initialize(Poco::Util::Application& app) {}

void palm::fig::Application::defineOptions(Poco::Util::OptionSet& options) {
  options.addOption(
      Poco::Util::Option("help", "h",
                         "display help information on command line arguments",
                         false)
          .repeatable(false)
          .required(false)
          .callback(Poco::Util::OptionCallback<Application>(
              this, &Application::handleHelp)));
  options.addOption(Poco::Util::Option("debug", "d", "run on debug mode", false)
                        .repeatable(false)
                        .required(false)
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleDebug)));
  options.addOption(Poco::Util::Option("version", "v", "show version", false)
                        .repeatable(false)
                        .required(false)
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleVersion)));
  options.addOption(Poco::Util::Option("config", "c",
                                       "load configuration data from a file",
                                       false)
                        .repeatable(true)
                        .required(false)
                        .argument("file")
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleConfig)));
}
void palm::fig::Application::initialize(Poco::Util::Application& app) {}

int palm::fig::Application::main(const std::vector<std::string>& args) {
  if (this->stopped) {
    return Application::EXIT_OK;
  }
  BOOST_LOG_TRIVIAL(info) << "start " << PALM_GIT_VERSION << "("
                          << PALM_BUILD_TIME << ")";
  BOOST_LOG_TRIVIAL(info) << "load configuration from " << this->config_file;
  return Application::EXIT_OK;
}

void palm::fig::Application::handleDebug(const std::string& name,
                                         const std::string& value) {}
void palm::fig::Application::handleHelp(const std::string& name,
                                        const std::string& value) {
  Poco::Util::HelpFormatter fmt(options());
  fmt.setCommand(commandName());
  fmt.setUsage("OPTIONS");
  fmt.setHeader(PALM_PROJECT_DESCRIPTION);
  fmt.format(std::cout);
  this->stopped = true;
}
void palm::fig::Application::handleConfig(const std::string& name,
                                          const std::string& value) {
  this->config_file = value;
}

void palm::fig::Application::handleVersion(const std::string& name,
                                           const std::string& value) {
  std::cout << PALM_GIT_VERSION << "(" << PALM_BUILD_TIME << ")" << std::endl;
  this->stopped = true;
}
