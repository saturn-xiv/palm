#include "aloe/s3.hpp"
#include "palm/s3.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

void aloe::s3::sync(const toml::table& config, const std::string& source,
                    const std::string& destination) {
  spdlog::info("sync s3 from {} to {}", source, destination);
  // TODO
}
void aloe::s3::sync(const toml::table& config, const std::string& source,
                    const std::string& destination,
                    const std::string& file_list) {
  spdlog::info("sync s3 from {} to {} by files list {}", source, destination,
               file_list);
  // TODO
}
void aloe::s3::dump(const toml::table& config, const std::string& host) {
  std::filesystem::path file("bla");
  spdlog::info("dump s3 {} to {}", host, file.string());
  //   TODO
}
void aloe::s3::restore(const toml::table& config, const std::string& host,
                       const std::string& filename) {
  spdlog::info("restore s3 {} to {}", filename, host);
  //   TODO
}
