#include "palm/i18n.hpp"

void palm::I18n::load(const std::filesystem::path& root) {
  const std::lock_guard<std::mutex> lock(this->locker);
  this->generator.add_messages_path(root);
  //   gen.add_messages_domain("auth");
  //   gen.add_messages_domain("rbac");
  this->generator.add_messages_domain("nut");
}
