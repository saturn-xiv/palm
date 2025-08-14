#include "phlox/application.hpp"
#include "phlox/filesystem.hpp"

void phlox::Application::fs_watcher(
    const toml::table& config, bool stdin,
    const std::set<std::string>& original_files) {
  if (palm::is_stopped()) {
    return;
  }

  std::shared_ptr<palm::opensearch::Client> search = this->opensearch(config);

  phlox::monitoring::LoggingScratcher scratcher;

  if (stdin) {
    spdlog::info("listen from STDIN stream");
    std::shared_ptr<phlox::monitoring::logging::Source> it =
        std::make_shared<phlox::monitoring::logging::StdinSource>();
    scratcher.register_(it);
  }
  if (!original_files.empty()) {
    std::shared_ptr<phlox::monitoring::logging::FilesystemNotify> it =
        std::make_shared<phlox::monitoring::logging::FilesystemNotify>();
    for (const auto& file : original_files) {
      it->register_(file);
    }

    scratcher.register_(it);
  }
  scratcher.launch(search);
}
