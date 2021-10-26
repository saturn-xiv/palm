#define BOOST_TEST_MODULE i18n
#include <boost/test/unit_test.hpp>

#include "palm/i18n.hpp"

BOOST_AUTO_TEST_CASE(tr) {
  boost::locale::generator gen;
  gen.add_messages_path(".");
  gen.add_messages_domain("palm");
  for (const auto it : {"en_US", "zh_CN", "zh_TW"}) {
    // setlocale(LC_ALL, it);
    // bindtextdomain("palm", "./");
    // textdomain("palm");
    // std::cout.imbue(std::locale(it));
    std::cout << it << ": " << palm::tr(it, "Hello, World!") << std::endl;
  }
}
