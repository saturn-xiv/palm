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

std::string filename_without_extension(const std::filesystem::path& name);

std::string timestamp();
std::string uuid();

namespace random {
std::string string(std::string::size_type length);
std::vector<unsigned char> bytes(const size_t length);
}  // namespace random

namespace hex {
std::string to(const std::vector<unsigned char>& buffer);
std::vector<unsigned char> from(const std::string& buffer);
}  // namespace hex

}  // namespace palm
