#pragma once

#include "palm/cache.hpp"
#include "palm/crypto.hpp"
#include "palm/i18n.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"

namespace palm {
namespace nut {
inline bool is_stopped() { return std::filesystem::exists(".stop"); }
void init_logger(bool debug);
}  // namespace nut
}  // namespace palm
