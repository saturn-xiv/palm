#include "pansy/application.hpp"
#include "pansy/services.hpp"

#include "cstdlib"

#include <boost/exception/all.hpp>

#include <spdlog/spdlog.h>

int main(int argc, char** argv) {
  //   pansy::Application app;
  try {
    // app.launch(argc, argv);
  } catch (...) {
    spdlog::error("{}", boost::current_exception_diagnostic_information());
  }
  return EXIT_SUCCESS;
}
