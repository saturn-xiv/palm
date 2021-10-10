#pragma once

#include "palm/env.hpp"

#include <sys/utsname.h>

#include <Poco/Util/Application.h>

namespace palm {

class Application : public Poco::Util::Application {
 protected:
  void defineOptions(Poco::Util::OptionSet& options) override;
  int main(const std::vector<std::string>& args) override;
  virtual int launch() = 0;
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
std::string current_user();
::utsname os();

std::string filename_without_extension(const std::filesystem::path& name);

}  // namespace palm
