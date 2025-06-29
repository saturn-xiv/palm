#pragma once

#include "palm/wisteria.hpp"
#include "questionnaire.grpc.pb.h"

namespace palm {
namespace questionnaire {
namespace services {
class FormServiceImpl final : public palm::questionnaire::v1::Form::Service {};
}  // namespace services
}  // namespace questionnaire
}  // namespace palm
