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
};

}  // namespace iris
