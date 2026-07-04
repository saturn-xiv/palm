#include "loquat/application.hpp"

#include <boost/exception/diagnostic_information.hpp>

#include <spdlog/spdlog.h>

int main(int argc, char** argv) {
  try {
    loquat::Application app;
    app.launch(argc, argv);
  } catch (...) {
    spdlog::error(boost::current_exception_diagnostic_information());
  }
}
