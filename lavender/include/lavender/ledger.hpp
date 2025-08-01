#pragma once

#include "lavender/portal.hpp"
#include "ledger.grpc.pb.h"

namespace lavender {
namespace ledger {
namespace services {
class BookServiceImpl final : public palm::ledger::v1::Book::Service {};
}  // namespace services
}  // namespace ledger
}  // namespace lavender
