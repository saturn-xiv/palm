#pragma once

#include "palm/portal.hpp"
#include "survey.grpc.pb.h"

namespace palm {
namespace survey {
namespace services {
class FormServiceImpl final : public palm::survey::v1::Form::Service {};
}  // namespace services
}  // namespace survey
}  // namespace palm
