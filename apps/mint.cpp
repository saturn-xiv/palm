
#include "palm/mint.hpp"
#include "palm/version.hpp"

#include <cstdlib>

#include <boost/log/trivial.hpp>

int main() {
  BOOST_LOG_TRIVIAL(info) << "start " << PALM_GIT_VERSION << "("
                          << PALM_BUILD_TIME << ")";
  return EXIT_SUCCESS;
}
