#pragma once

#include "monitoring.grpc.pb.h"
#include "palm/jwt.hpp"
#include "palm/search.hpp"
#include "palm/theme.hpp"

namespace palm {
void mount(httplib::Server& server, palm::Theme& theme,
           std::shared_ptr<palm::Jwt> jwt,
           std::shared_ptr<palm::opensearch::Client> search);
}
