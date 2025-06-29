#pragma once

#include "basil/http.hpp"

#include <format>

#include <inja/inja.hpp>

namespace basil {

class Theme {
 public:
  Theme(const std::string& folder, const nlohmann::json& global = {})
      : _env({std::format("{}/views/", folder)}), _global(global) {}

  inline std::string render(const std::string& tpl, nlohmann::json& data) {
    data.update(this->_global);
    return this->_env.render_file(tpl, data);
  }

 private:
  inja::Environment _env;
  nlohmann::json _global;
};
}  // namespace basil
