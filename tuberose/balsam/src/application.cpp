#include "balsam/application.hpp"
#include "palm/version.hpp"

balsam::Application::Application(int argc, char** argv) {
  const std::string version = palm::GIT_VERSION + "(" + palm::BUILD_TIME + ")";
  {
    // spdlog::set_level(program.get<bool>("--debug") ? spdlog::level::debug
    //                                                : spdlog::level::info);
    spdlog::debug("run on debug mode {}", version);
  }
}
