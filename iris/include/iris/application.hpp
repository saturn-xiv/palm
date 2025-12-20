#pragma once

#include <string>

namespace iris {
class Application {
 public:
  Application() {}
  int launch(int argc, char** argv) const;

 private:
  void dump(const std::string& input, const std::string& output, bool compress,
            size_t keep) const;
  // https://man.archlinux.org/man/systemd.timer.5
  // https://man.archlinux.org/man/systemd.time.7.en
  void generate_timer(const std::string& name) const;
};

}  // namespace iris
