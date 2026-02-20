#pragma once

#include "cms.grpc.pb.h"
#include "tulip/portal.hpp"

namespace tulip {
namespace cms {
namespace controllers {
namespace pages {
std::shared_ptr<palm::cms::v1::ShowPageHtml> show(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const std::string& permalink);
std::shared_ptr<palm::cms::v1::IndexPageResponse_Item> show(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const palm::portal::v1::IdRequest& req);
std::shared_ptr<palm::cms::v1::IndexPageHtml> index(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const std::shared_ptr<palm::portal::v1::Page> page);
std::shared_ptr<palm::cms::v1::IndexPageResponse> index(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const palm::portal::v1::Page& req);
}  // namespace pages
}  // namespace controllers
}  // namespace cms
}  // namespace tulip
