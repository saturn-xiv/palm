#pragma once

#include <string>

#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS 1
#include <toml++/toml.hpp>

namespace aloe {
namespace s3 {
void sync(const std::string& source, const std::string& destination,
          const std::string& file_list);
void sync(const std::string& source, const std::string& destination);
void dump(const std::vector<std::string>& hosts, bool zip = false);
void restore(const std::string& host, const std::string& tar_file);
void restore(const std::string& host, const std::string& tar_file,
             const std::string& file_list);
struct Object {
  std::string name;
  size_t size;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Object, name)
};
struct Bucket {
  std::string name;
  std::vector<Object> objects;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Bucket, name, objects)
};
struct Host {
  std::vector<Bucket> buckets;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Host, buckets)
};
struct File {
  std::string bucket;
  std::string object;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(File, bucket, object)
};
inline static const std::string ROOTFS = "rootfs";
inline static const std::string INDEX = "index.json";
}  // namespace s3
}  // namespace aloe
