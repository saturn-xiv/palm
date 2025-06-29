#include "palm/monitor.hpp"

#include <boost/current_function.hpp>

void palm::monitor::mount(httplib::Server& server, palm::Theme& theme,
                          std::shared_ptr<palm::Jwt> jwt,
                          std::shared_ptr<palm::opensearch::Client> search) {
  BOOST_LOG_TRIVIAL(debug) << BOOST_CURRENT_FUNCTION;
  // TODO
}
