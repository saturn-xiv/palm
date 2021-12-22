#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace fig {
class Application {
 public:
  Application(int argc, char** argv);

 private:
  void rpc(const boost::property_tree::ptree& config);
  void web(const boost::property_tree::ptree& config);
  void worker(const boost::property_tree::ptree& config);
};
}  // namespace fig
}  // namespace palm
