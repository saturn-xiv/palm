#pragma once

#include <cstdint>
#include <string>

namespace loquat {
class Application {
 public:
  Application() {}

  void launch(int argc, char** argv) const;

 private:
};
}  // namespace loquat
