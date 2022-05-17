#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

#include <iostream>

BOOST_AUTO_TEST_CASE(queries) {
  const auto tree = palm::orm::queries("db");

  for (const auto it : {"schema-migrations.latest", "locales.count"}) {
    const auto sql = tree.get<std::string>(it);
    std::cout << it << ": " << sql << std::endl;
  }
}
