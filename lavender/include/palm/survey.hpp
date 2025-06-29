#pragma once

#include "palm/portal.hpp"
#include "survey.grpc.pb.h"

namespace palm {
namespace questionnaire {
namespace services {
class FormServiceImpl final : public palm::questionnaire::v1::Form::Service {};
}  // namespace services
}  // namespace questionnaire
}  // namespace palm
