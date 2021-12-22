#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace fig {
class Application {
 private:
  void rpc();
  void web();
  void worker();
};
}  // namespace fig
}  // namespace palm
