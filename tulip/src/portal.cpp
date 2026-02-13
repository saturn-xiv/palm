#include "tulip/portal.hpp"

#include <cstdlib>

void tulip::init(bool debug) {
  Poco::Logger& logger = Poco::Logger::get("portal");
  Poco::Logger::root().setLevel(debug ? Poco::Message::PRIO_DEBUG
                                      : Poco::Message::PRIO_INFORMATION);
  logger.debug("run on debug mode");
  if (sodium_init() < 0) {
    logger.error("libsodium couldn't be initialized");
    std::exit(EXIT_FAILURE);
  }
}
