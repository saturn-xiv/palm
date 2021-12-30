#define BOOST_TEST_MODULE i18n
#include <boost/test/unit_test.hpp>

#include <palm/i18n.hpp>

#include <iostream>

BOOST_AUTO_TEST_CASE(tr) {
  auto& i18n = palm::I18n::instance();
  i18n.load("locales");
  const char* locales[] = {"en_US.UTF-8", "zh_CN.UTF-8", "zh_TW.UTF-8"};

  for (const auto it : locales) {
    const std::locale lc = i18n.get(it);
    std::cout << it << " " << lc.name() << ": "
              << boost::locale::translate("languages.en-us").str(lc, "nut")
              << std::endl;
  }
}
