#pragma once

#include <filesystem>
#include <locale>
#include <mutex>
#include <string>

#define BOOST_LOCALE_HIDE_AUTO_PTR
#include <boost/locale.hpp>
namespace palm {
// https://www.boost.org/doc/libs/1_74_0/libs/locale/doc/html/messages_formatting.html
class I18n {
 public:
  I18n(I18n const&) = delete;
  void operator=(I18n const&) = delete;

  static I18n& instance() {
    static I18n it;
    return it;
  }
  void load(const std::filesystem::path& root);
  inline std::locale get(const std::string& s) const {
    return this->generator(s);
  }

 private:
  I18n() {}
  std::mutex locker;
  boost::locale::generator generator;
};

}  // namespace palm
