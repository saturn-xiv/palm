#include "env.hpp"
#include "loquat/version.hpp"

int loquat::init(bool debug) {
  {
    spdlog::set_level(debug ? spdlog::level::debug : spdlog::level::info);
    spdlog::debug("run on debug mode {}", version);
    spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
  }
  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      spdlog::error("failed to register tink");
      return EXIT_FAILURE;
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      spdlog::error("failed to register tink-jwt");
      return EXIT_FAILURE;
    }
  }
}
