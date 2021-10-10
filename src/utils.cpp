#include "palm/utils.hpp"

#include <sys/reboot.h>
#include <unistd.h>

#include <Poco/AutoPtr.h>
#include <Poco/ConsoleChannel.h>
#include <Poco/FormattingChannel.h>
#include <Poco/PatternFormatter.h>
#include <Poco/Util/HelpFormatter.h>

void palm::Application::defineOptions(Poco::Util::OptionSet& options) {
  Poco::Util::Application::defineOptions(options);
  options.addOption(Poco::Util::Option("help", "h", "display help information")
                        .required(false)
                        .repeatable(false)
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleHelp)));

  options.addOption(Poco::Util::Option("debug", "d", "run in debug mode")
                        .required(false)
                        .repeatable(false)
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleDebug)));
  options.addOption(Poco::Util::Option("version", "v", "show version")
                        .required(false)
                        .repeatable(false)
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleVersion)));
}
void palm::Application::handleDebug(const std::string& name,
                                    const std::string& value) {
  this->debug = true;
}
void palm::Application::handleVersion(const std::string& name,
                                      const std::string& value) {
  std::cout << PALM_GIT_VERSION << "(" << PALM_BUILD_TIME << ")" << std::endl;
  this->stopped = true;
  stopOptionsProcessing();
}
void palm::Application::handleHelp(const std::string& name,
                                   const std::string& value) {
  Poco::Util::HelpFormatter helpFormatter(options());
  helpFormatter.setCommand(commandName());
  helpFormatter.setUsage("OPTIONS");
  helpFormatter.setHeader(this->description());
  helpFormatter.format(std::cout);
  this->stopped = true;
  stopOptionsProcessing();
}

int palm::Application::main(const std::vector<std::string>& args) {
  palm::set_logger(this->debug);
  if (this->stopped) {
    return Poco::Util::Application::EXIT_OK;
  }

  return this->launch();
}

void palm::reboot() {
  ::sync();
  ::reboot(RB_AUTOBOOT);
}

std::string palm::current_user() { return ::getlogin(); }

/**
 * https://pocoproject.org/docs/Poco.html#14756
 * https://pocoproject.org/docs/Poco.PatternFormatter.html
 */
void palm::set_logger(bool debug) {
  Poco::AutoPtr<Poco::ConsoleChannel> ch(new Poco::ConsoleChannel);
  Poco::AutoPtr<Poco::PatternFormatter> pf(new Poco::PatternFormatter);

  pf->setProperty("pattern", "%Y-%m-%d %H:%M:%S.%i[%T-%I]%q: %t");
  Poco::AutoPtr<Poco::FormattingChannel> pfc(
      new Poco::FormattingChannel(pf, ch));
  Poco::Logger& logger = Poco::Logger::root();
  logger.setLevel(debug ? Poco::Message::PRIO_DEBUG
                        : Poco::Message::PRIO_INFORMATION);
  logger.setChannel(pfc);
  poco_debug(logger, "run in debug mode");
}

std::string palm::filename_without_extension(
    const std::filesystem::path& file) {
  const auto name = file.filename().string();
  const size_t dot = name.find_first_of(".");
  return dot == std::string::npos ? name : name.substr(0, dot);
}

::utsname palm::os() {
  ::utsname it;
  ::uname(&it);
  return it;
}
