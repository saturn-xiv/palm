#include "tulip/cms.hpp"

std::shared_ptr<palm::cms::v1::ShowPageHtml>
tulip::cms::controllers::pages::show(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const std::string& permalink) {
  auto res = std::make_shared<palm::cms::v1::ShowPageHtml>();
  // TODO
  return res;
}
std::shared_ptr<palm::cms::v1::IndexPageHtml>
tulip::cms::controllers::pages::index(
    tulip::portal::Context& ctx, std::shared_ptr<palm::portal::v1::Session> ss,
    const std::shared_ptr<palm::portal::v1::Page> page) {
  auto res = std::make_shared<palm::cms::v1::IndexPageHtml>();
  // TODO
  return res;
}
std::shared_ptr<palm::cms::v1::IndexPageResponse_Item>
tulip::cms::controllers::pages::show(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const palm::portal::v1::IdRequest& req) {
  auto res = std::make_shared<palm::cms::v1::IndexPageResponse_Item>();
  // TODO
  res->set_id(123);
  res->set_title("hi");
  return res;
}
std::shared_ptr<palm::cms::v1::IndexPageResponse>
tulip::cms::controllers::pages::index(
    tulip::portal::Context& ctx,
    const std::shared_ptr<palm::portal::v1::Session> ss,
    const palm::portal::v1::Page& req) {
  auto res = std::make_shared<palm::cms::v1::IndexPageResponse>();
  // TODO
  return res;
}
