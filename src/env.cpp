#include "palm/env.hpp"

#include <boost/log/expressions.hpp>
#include <boost/log/trivial.hpp>

void palm::init_logger(bool debug) {
  boost::log::core::get()->set_filter(
      boost::log::trivial::severity >=
      (debug ? boost::log::trivial::debug : boost::log::trivial::info));
  BOOST_LOG_TRIVIAL(debug) << "run on debug mode";
}
