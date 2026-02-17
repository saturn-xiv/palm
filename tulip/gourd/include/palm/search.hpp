#pragma once

#include <cpr/cpr.h>
#include <spdlog/spdlog.h>
#include <toml++/toml.hpp>

namespace {

class OpenSearch {
 public:
  OpenSearch(const toml::table& config)
      : _host(config["host"].value_or<std::string>("127.0.0.1")),
        _port(config["port"].value_or<uint16_t>(9200)),
        _namespace(config["namespace"].value_or<std::string>("")) {}
  OpenSearch(const std::string& host = "127.0.0.1", uint16_t port = 9200,
             const std::string& namespace_ = "")
      : _host(host), _port(port), _namespace(namespace_) {}

 private:
  std::string _host;
  uint16_t _port;
  std::string _namespace;
};
}  // namespace
