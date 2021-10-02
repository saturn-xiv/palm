#include "palm/utils.hpp"

#include <sys/reboot.h>

#include <Poco/AutoPtr.h>
#include <Poco/ConsoleChannel.h>
#include <Poco/FormattingChannel.h>
#include <Poco/PatternFormatter.h>
#include <Poco/UUID.h>
#include <Poco/UUIDGenerator.h>
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
  this->launch();
  return Poco::Util::Application::EXIT_OK;
}

void palm::reboot() {
  ::sync();
  ::reboot(RB_AUTOBOOT);
}

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

// https://en.cppreference.com/w/cpp/io/manip/put_time
std::string palm::timestamp() {
  auto now = std::chrono::system_clock::now();
  auto itt = std::chrono::system_clock::to_time_t(now);
  std::ostringstream ss;
  ss << std::put_time(gmtime(&itt), "%Y%m%d%H%M%S");
  return ss.str();
}

std::string palm::uuid() {
  thread_local static Poco::UUIDGenerator& gen =
      Poco::UUIDGenerator::defaultGenerator();
  Poco::UUID it = gen.create();
  return it.toString();
}

std::string palm::random::string(std::string::size_type length) {
  thread_local static auto& chrs =
      R"(0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ)";

  thread_local static std::mt19937 rg{std::random_device{}()};
  thread_local static std::uniform_int_distribution<std::string::size_type>
      pick(0, sizeof(chrs) - 2);

  std::string s;

  s.reserve(length);

  while (length--) s += chrs[pick(rg)];

  return s;
}

std::vector<unsigned char> palm::random::bytes(const size_t length) {
  thread_local static std::independent_bits_engine<std::default_random_engine,
                                                   CHAR_BIT, unsigned char>
      rbe;
  std::vector<unsigned char> buffer(length);
  std::generate(std::begin(buffer), std::end(buffer), std::ref(rbe));
  return buffer;
}

std::string palm::hex::to(const std::vector<unsigned char>& buffer) {
  std::stringstream ss;
  ss << std::hex << std::setfill('0');
  for (const auto it : buffer) {
    ss << std::setw(2) << static_cast<unsigned>(it);
  }
  return ss.str();
}
std::vector<unsigned char> palm::hex::from(const std::string& buffer) {
  std::vector<unsigned char> items;
  for (auto i = 0; i < buffer.length(); i += 2) {
    const auto it = buffer.substr(i, 2);
    items.push_back(static_cast<unsigned char>(strtol(it.c_str(), NULL, 16)));
  }
  return items;
}
