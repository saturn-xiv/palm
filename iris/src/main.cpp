#include "iris/application.hpp"

#include <spdlog/spdlog.h>

int main(int argc, char** argv) {
  iris::Application app;
  try {
    return app.launch(argc, argv);
  } catch (const std::exception& ex) {
    spdlog::error("{}", ex.what());
  } catch (...) {
    spdlog::error("an unknown exception occurred");
  }
  return EXIT_FAILURE;
}
