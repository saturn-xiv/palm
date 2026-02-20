#include "palm/search.hpp"

std::shared_ptr<palm::opensearch::responses::info::Item>
palm::opensearch::Config::info() const {
  cpr::Response res = cpr::Get(cpr::Url{this->url("")});
  if (res.status_code != 200) {
    spdlog::error("{} {}", res.status_code, res.text);
    return nullptr;
  }
  auto obj = std::make_shared<palm::opensearch::responses::info::Item>();
  nlohmann::json js = nlohmann::json::parse(res.text);
  nlohmann::from_json(js, *obj);
  return obj;
}
