#pragma once

#include "basil/wisteria.hpp"
#include "questionnaire.grpc.pb.h"

namespace basil {
namespace questionnaire {
namespace services {
class FormServiceImpl final : public basil::questionnaire::v1::Form::Service {};
}  // namespace services
}  // namespace questionnaire
}  // namespace basil
