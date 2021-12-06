#include <cstdlib>

#include "palm/fig.hpp"

#include <grpcpp/ext/proto_server_reflection_plugin.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>

int main() {
  BOOST_LOG_TRIVIAL(info) << "start " << PALM_GIT_VERSION << "("
                          << PALM_BUILD_TIME << ")";
  return EXIT_SUCCESS;
}
