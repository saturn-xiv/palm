#pragma once

#include "ledger.grpc.pb.h"
#include "palm/portal.hpp"

namespace palm {
namespace ledger {
namespace services {
class BookServiceImpl final : public palm::ledger::v1::Book::Service {};
}  // namespace services
}  // namespace ledger
}  // namespace palm
