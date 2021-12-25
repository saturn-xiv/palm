#pragma once

#include <boost/property_tree/ptree.hpp>

namespace palm {
namespace fig {
class Application {
 public:
  Application(int argc, char** argv);

 private:
  void rpc(const boost::property_tree::ptree& config) const;
  void web(const boost::property_tree::ptree& config) const;
  void worker(const boost::property_tree::ptree& config) const;
};
}  // namespace fig
}  // namespace palm
