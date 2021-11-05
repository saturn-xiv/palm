#define BOOST_TEST_MODULE lunar
#include <boost/test/unit_test.hpp>

#include "palm/lunar.hpp"

BOOST_AUTO_TEST_CASE(five_xing) {}

BOOST_AUTO_TEST_CASE(out) {
  for (palm::lunar::六十四卦 it = palm::lunar::六十四卦::乾;
       it != palm::lunar::六十四卦::未濟;
       it = static_cast<palm::lunar::六十四卦>(static_cast<int>(it) + 1)) {
    std::cout << it << std::endl;
    // if ((i % 8) == 7) {
    //   std::cout << std::endl;
    // }
  }
  //   for (int i = palm::lunar::六十四卦::乾; i != palm::lunar::六十四卦::未濟;
  //        i++) {
  //     std::cout << i << " ";
  //     if ((i % 8) == 7) {
  //       std::cout << std::endl;
  //     }
  //   }
}
