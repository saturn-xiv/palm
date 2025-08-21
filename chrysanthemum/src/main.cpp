#include "chrysanthemum/application.hpp"

#include <cstdlib>

#include <boost/exception/diagnostic_information.hpp>

#include <spdlog/spdlog.h>

int main(int argc, char** argv) {
  chrysanthemum::Application app;
  try {
    app.launch(argc, argv);
    return EXIT_SUCCESS;
  } catch (...) {
    spdlog::error("{}", boost::current_exception_diagnostic_information());
    return EXIT_FAILURE;
  }
}
