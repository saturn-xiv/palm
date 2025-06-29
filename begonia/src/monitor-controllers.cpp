#include "basil/monitor.hpp"

#include <boost/current_function.hpp>

void basil::monitor::mount(httplib::Server& server, basil::Theme& theme,
                           std::shared_ptr<basil::Jwt> jwt,
                           std::shared_ptr<basil::opensearch::Client> search) {
  BOOST_LOG_TRIVIAL(debug) << BOOST_CURRENT_FUNCTION;
  // TODO
}
