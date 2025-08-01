#pragma once

#include "lavender/portal.hpp"
#include "survey.grpc.pb.h"

namespace lavender {
namespace survey {
namespace services {
class FormServiceImpl final : public palm::survey::v1::Form::Service {};
}  // namespace services
}  // namespace survey
}  // namespace lavender
