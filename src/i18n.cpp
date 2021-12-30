#include "palm/i18n.hpp"

void palm::I18n::load(const std::filesystem::path& root) {
  const std::lock_guard<std::mutex> lock(this->locker);
  this->generator.add_messages_path(root);
  const char* plugins[] = {"nut"};
  for (const auto it : plugins) {
    this->generator.add_messages_domain(it);
  }
}
