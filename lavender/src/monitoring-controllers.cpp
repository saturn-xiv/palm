#include "palm/monitoring.hpp"

#include <boost/current_function.hpp>

void palm::monitoring::mount(httplib::Server& server, palm::Theme& theme,
                             std::shared_ptr<palm::Jwt> jwt,
                             std::shared_ptr<palm::opensearch::Client> search) {
  spdlog::debug("{}", BOOST_CURRENT_FUNCTION);
  // TODO
}
