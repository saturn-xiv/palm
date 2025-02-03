#include "marguerite/utils.hpp"

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include "boost/date_time/posix_time/posix_time.hpp"

std::string marguerite::uuid() {
  thread_local static boost::uuids::random_generator rng;
  const boost::uuids::uuid it = rng();
  //   return boost::lexical_cast<std::string>(it);
  return boost::uuids::to_string(it);
}

std::string marguerite::timestamp() {
  boost::posix_time::ptime now =
      boost::posix_time::second_clock::universal_time();
  static std::locale loc(std::cout.getloc(),
                         new boost::posix_time::time_facet("%Y%m%d%H%M%S"));

  std::stringstream ss;
  ss.imbue(loc);
  ss << now;
  return ss.str();
}
