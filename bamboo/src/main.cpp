#include "palm/application.hpp"

#include "cstdlib"

#include <boost/exception/all.hpp>

#include <spdlog/spdlog.h>

int main(int argc, char** argv) {
  palm::bamboo::Application app;
  try {
    app.launch(argc, argv);
  } catch (...) {
    spdlog::error("{}", boost::current_exception_diagnostic_information());
  }
  return EXIT_SUCCESS;
}
