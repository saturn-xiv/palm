#pragma once

#include "palm/wisteria.hpp"
#include "bookkeeper.grpc.pb.h"

namespace palm {
namespace bookkeeper {
namespace services {
class BookServiceImpl final : public palm::bookkeeper::v1::Book::Service {};
}  // namespace services
}  // namespace bookkeeper
}  // namespace palm
