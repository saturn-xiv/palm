#pragma once

#include "basil/wisteria.hpp"
#include "bookkeeper.grpc.pb.h"

namespace basil {
namespace bookkeeper {
namespace services {
class BookServiceImpl final : public basil::bookkeeper::v1::Book::Service {};
}  // namespace services
}  // namespace bookkeeper
}  // namespace basil
