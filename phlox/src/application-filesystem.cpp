#include "phlox/application.hpp"
#include "phlox/filesystem.hpp"

void phlox::Application::fs_watcher(
    const toml::table& config, bool stdin,
    const std::vector<std::string>& original_files) {
  if (palm::is_stopped()) {
    return;
  }

  std::shared_ptr<palm::opensearch::Client> search =
      std::make_shared<palm::opensearch::Client>(config);
  {
    auto res = search->cluster_health();
    spdlog::debug("{} {}", res->cluster_name, res->status);
  }
  if (!search->index_exists<phlox::monitoring::logging::Item>()) {
    const auto props = phlox::monitoring::logging::Item::properties();
    search->create_index<phlox::monitoring::logging::Item>(2, 1, props);
  }
  phlox::monitoring::LoggingScratcher scratcher;

  if (stdin) {
    spdlog::info("listen from STDIN stream");
    std::shared_ptr<phlox::monitoring::logging::Source> it =
        std::make_shared<phlox::monitoring::logging::StdinSource>();
    scratcher.register_(it);
  }
  {
    std::shared_ptr<phlox::monitoring::logging::FilesystemNotify> it =
        std::make_shared<phlox::monitoring::logging::FilesystemNotify>();
    const std::set<std::string> items(original_files.begin(),
                                      original_files.end());
    for (const auto& file : items) {
      it->register_(file);
    }

    scratcher.register_(it);
  }
  scratcher.launch(search);
}
