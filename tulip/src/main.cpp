#include "tulip/application.hpp"

#include <cstdlib>

#include <spdlog/spdlog.h>
#include <boost/exception/diagnostic_information.hpp>

int main(int argc, char** argv) {
  tulip::Application app;
  try {
    return app.launch(argc, argv);
  } catch (...) {
    spdlog::error(boost::current_exception_diagnostic_information());
  }
  return EXIT_FAILURE;
}
