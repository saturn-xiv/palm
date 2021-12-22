#pragma once

#include <filesystem>
#include <fstream>
#include <optional>
#include <ostream>
#include <string>

#include <soci/soci.h>

namespace palm {
inline std::string file2str(const std::filesystem::path& file) {
  std::ifstream fs;
  std::ios_base::iostate mask = fs.exceptions() | std::ios::failbit;
  fs.exceptions(mask);
  fs.open(file);
  return std::string((std::istreambuf_iterator<char>(fs)),
                     std::istreambuf_iterator<char>());
}
void init_logger(bool debug = false);
}  // namespace palm

namespace soci {

// simple fall-back for std::optional
template <typename T>
struct type_conversion<std::optional<T> > {
  typedef typename type_conversion<T>::base_type base_type;

  static void from_base(base_type const& in, indicator ind,
                        std::optional<T>& out) {
    if (ind == i_null) {
      out.reset();
    } else {
      T tmp = T();
      type_conversion<T>::from_base(in, ind, tmp);
      out = tmp;
    }
  }

  static void to_base(std::optional<T> const& in, base_type& out,
                      indicator& ind) {
    if (in) {
      type_conversion<T>::to_base(in.value(), out, ind);
    } else {
      ind = i_null;
    }
  }
};

}  // namespace soci
