#pragma once

#include "palm/env.hpp"

#include <Poco/Util/Application.h>

namespace palm {

class Application : public Poco::Util::Application {
 protected:
  void defineOptions(Poco::Util::OptionSet& options) override;
  int main(const std::vector<std::string>& args) override;
  virtual void launch() = 0;
  virtual std::string description() = 0;

 private:
  void handleDebug(const std::string& name, const std::string& value);
  void handleHelp(const std::string& name, const std::string& value);
  void handleVersion(const std::string& name, const std::string& value);
  bool stopped;
  bool debug;
};

void set_logger(bool debug);

inline bool is_stopped() { return std::filesystem::exists(".stop"); }

void reboot();

}  // namespace palm
