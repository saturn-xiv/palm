#include "palm/orm.hpp"

#include <boost/log/trivial.hpp>
#include <boost/property_tree/ini_parser.hpp>

void palm::orm::Query::load(const std::filesystem::path& root) {
  const std::lock_guard<std::mutex> lock(this->locker);
  for (const auto& it : std::filesystem::directory_iterator(root / "queries")) {
    const std::string file = it.path().string();
    BOOST_LOG_TRIVIAL(info) << "load query file from " << file;
    boost::property_tree::ptree tree;
    boost::property_tree::read_ini(file, tree);
    this->trees[file] = tree;
  }
}
