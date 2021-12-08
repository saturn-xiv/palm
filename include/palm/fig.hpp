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
  void uninitialize() override;

 private:
};

class Web : public Poco::Util::Subsystem {
 public:
  const char* name() const override { return "web"; }
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override;

 private:
};

class Worker : public Poco::Util::Subsystem {
 public:
  const char* name() const override { return "worker"; }
  void defineOptions(Poco::Util::OptionSet& options) override;
  void initialize(Poco::Util::Application& app) override;
  void uninitialize() override;

 private:
};

class Application : public Poco::Util::Application {
 public:
 private:
};
}  // namespace fig
}  // namespace palm
