#define BOOST_TEST_MODULE twilio
#include <boost/test/unit_test.hpp>

#include "palm/utils.hpp"

namespace palm {}  // namespace palm

BOOST_AUTO_TEST_CASE(iso8601) {
  std::time_t now = std::time(nullptr);
  std::tm* tm1 = std::localtime(&now);
  std::cout << "local time: " << std::asctime(tm1);
  const auto js = palm::iso8601::to(tm1);
  std::cout << "iso8601: " << js << std::endl;
  const auto tm2 = palm::iso8601::from(js);
  std::cout << "from parse: " << std::asctime(&tm2);

  BOOST_TEST(tm1->tm_sec == tm2.tm_sec);
  BOOST_TEST(tm1->tm_min == tm2.tm_min);
}

BOOST_AUTO_TEST_CASE(std_tm) {
  std::cout << date::format(PALM_STDTM_ISO8601_FORMAT,
                            std::chrono::floor<std::chrono::seconds>(
                                std::chrono::system_clock::now()))
            << std::endl;
  std::time_t now = std::time(nullptr);
  {
    std::cout << "epoch time: " << now << std::endl;
    std::cout << "epoch time: " << std::ctime(&now);
  }
  {
    std::tm* ut = std::gmtime(&now);
    std::cout << "utc time: " << std::asctime(ut);
  }
  {
    std::tm* lt1 = std::localtime(&now);
    std::cout << "local time(asc): " << std::asctime(lt1);

    std::time_t tt1 = std::mktime(lt1);
    {
      std::cout << "epoch time: " << tt1 << std::endl;
      std::cout << "epoch time: " << std::ctime(&tt1);
    }
    BOOST_TEST(now == tt1);

    std::chrono::system_clock::time_point tp =
        std::chrono::system_clock::from_time_t(tt1);
    // std::chrono::seconds seconds = std::chrono::seconds{tt1};
    const std::string iso =
        date::format(PALM_STDTM_ISO8601_FORMAT,
                     std::chrono::floor<std::chrono::seconds>(tp));
    std::cout << "ISO8601: " << iso << std::endl;

    {
      date::local_seconds tp;
      {
        std::istringstream is(iso);
        is.exceptions(std::ios::failbit);
        is >> date::parse(PALM_STDTM_ISO8601_FORMAT, tp);
      }
    }
    // std::tm t2;

    // std::cout << "T2: " << std::asctime(&t2);

    // BOOST_TEST(*t1, t2);
  }
}
