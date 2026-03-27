#include "tulip/application.hpp"

#include <cstdlib>

#include <absl/log/initialize.h>
#include <google/protobuf/stubs/common.h>
#include <spdlog/spdlog.h>
#include <boost/exception/diagnostic_information.hpp>

int main(int argc, char** argv) {
  GOOGLE_PROTOBUF_VERIFY_VERSION;
  absl::InitializeLog();

  tulip::Application app;
  try {
    return app.launch(argc, argv);
  } catch (...) {
    spdlog::error(boost::current_exception_diagnostic_information());
  }
  return EXIT_FAILURE;
}
