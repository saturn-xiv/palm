#include "tulip/accounting.hpp"
#include "tulip/application.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"

#include <cstdlib>

#include <Poco/ConsoleChannel.h>

int main(int argc, char** argv) {
  Poco::AutoPtr<Poco::ConsoleChannel> channel(new Poco::ConsoleChannel);
  Poco::Logger::root().setChannel(channel);

  tulip::init(true);

  Poco::Logger& logger = Poco::Logger::get("main");
  logger.information("listening on tcp://0.0.0.0:8080");
  return EXIT_SUCCESS;
}
