#define BOOST_TEST_MODULE orm
#include <boost/test/unit_test.hpp>

#include "palm/orm.hpp"

#include <iostream>

#include <boost/property_tree/ini_parser.hpp>

BOOST_AUTO_TEST_CASE(queries) {
  const auto tree = palm::orm::queries("db");

  for (const auto it : {"schema-migrations.latest", "locales.count"}) {
    const auto sql = tree.get<std::string>(it);
    std::cout << it << ": " << sql << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(postgresql) {
  boost::property_tree::ptree tree;
  boost::property_tree::read_ini("config.ini", tree);
  palm::PostgreSQL pgsql(tree);
  auto pool = pgsql.open();
  {
    Poco::Data::Session db(pool->get());
    std::string version;
    {
      Poco::Data::Statement st(db);
      st << "SELECT version()", Poco::Data::Keywords::into(version),
          Poco::Data::Keywords::now;
      st.execute();
    }
    std::cout << "postgresql: " << version << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(mysql) {
  boost::property_tree::ptree tree;
  boost::property_tree::read_ini("config.ini", tree);
  palm::MySQL mysql(tree);
  auto pool = mysql.open();
  {
    Poco::Data::Session db(pool->get());
    std::string version;
    {
      Poco::Data::Statement st(db);
      st << "SELECT version()", Poco::Data::Keywords::into(version),
          Poco::Data::Keywords::now;
      st.execute();
    }
    std::cout << "mysql: " << version << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(sqlite3) {
  auto db = palm::sqlite3::open("tmp/test.db");
  std::string version;
  {
    Poco::Data::Statement st(db);
    st << "SELECT sqlite_version()", Poco::Data::Keywords::into(version),
        Poco::Data::Keywords::now;
    st.execute();
  }
  std::cout << "sqlite3: " << version << std::endl;
}
