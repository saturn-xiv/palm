#pragma once

#include "palm/orm.hpp"

#include <Poco/Util/Application.h>

namespace palm {
namespace fig {

class Rpc : public Poco::Util::Subsystem {
 public:
  const char* name() const override { return "rpc"; }
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override {}

 private:
};

class Web : public Poco::Util::Subsystem {
 public:
  const char* name() const override { return "web"; }
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override {}

 private:
};

class Worker : public Poco::Util::Subsystem {
 public:
  const char* name() const override { return "worker"; }
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override {}

 private:
};

class Application : public Poco::Util::Application {
 public:
  Application()
      : Poco::Util::Application(),
        stopped(false),
        config_file("config.properties") {}
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override {}
  int main(const std::vector<std::string>& args) override;

 private:
  void handleDebug(const std::string& name, const std::string& value);
  void handleHelp(const std::string& name, const std::string& value);
  void handleConfig(const std::string& name, const std::string& value);
  void handleVersion(const std::string& name, const std::string& value);

  bool stopped;
  std::filesystem::path config_file;
};
}  // namespace fig
}  // namespace palm
