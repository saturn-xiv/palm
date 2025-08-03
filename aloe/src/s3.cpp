#include "aloe/s3.hpp"
#include "palm/crypto.hpp"
#include "palm/s3.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

void aloe::s3::sync(const toml::table& config, const std::string& source,
                    const std::string& destination) {
  spdlog::info("sync s3 from {} to {}", source, destination);
  // TODO
}
void aloe::s3::sync(const toml::table& config, const std::string& source,
                    const std::string& destination, const std::string& list) {
  spdlog::info("sync s3 from {} to {} by files list {}", source, destination,
               list);
  // TODO
}

static inline void download(std::shared_ptr<palm::minio::Client> client,
                            const std::string& bucket,
                            const std::string& object,
                            const std::filesystem::path& rootfs) {
  spdlog::debug("download object {}/{}/{} to {}", client->base_url(), bucket,
                object, rootfs.string());
  std::filesystem::path folder = rootfs / bucket;
  if (!std::filesystem::exists(folder)) {
    spdlog::debug("create {}", folder.string());
    std::filesystem::create_directories(folder);
  }
  std::filesystem::path file = folder / object;
  if (std::filesystem::exists(file)) {
    spdlog::warn("file already exists {}", file.string());
    return;
  }
  {
    spdlog::info("create {}", file.string());
    std::ofstream output;
    output.open(file, std::ios::out);
    const auto ok = client->get_object(bucket, object, output);
    output.close();
    if (!ok) {
      spdlog::debug("remove file {}", file.string());
      std::filesystem::remove(file);
    }
  }
}

void aloe::s3::dump(const toml::table& config,
                    const std::vector<std::string>& hosts, bool compress) {
  const auto tmp = palm::timestamp();
  const auto tar = std::format("{}.tar", tmp);
  const auto zip = std::format("{}.tar.xz", tmp);

  spdlog::info("dump s3 {} to {}", boost::algorithm::join(hosts, ","), tmp);

  std::vector<Host> hosts_;
  {
    const auto root = std::filesystem::path(tmp) / ROOTFS;
    for (const auto& host : hosts) {
      Host host_;
      auto cli = palm::minio::Client::open(host);
      const auto buckets = cli->list_buckets();
      spdlog::debug("found {} buckets for {}", buckets.size(), host);
      for (const auto& bucket : buckets) {
        spdlog::debug("found bucket {}@{}", bucket, host);
        Bucket bucket_ = {.name = bucket};
        const auto objects = cli->list_objects(bucket);
        spdlog::debug("found {} objects for {}/{}", objects.size(), host,
                      bucket);
        for (const auto& [name, size] : objects) {
          spdlog::debug("found object {}/{}", bucket, name);
          download(cli, bucket, name, root);
          Object object_ = {.name = name, .size = size};
          bucket_.objects.push_back(object_);
        }
        host_.buckets.push_back(bucket_);
      }

      hosts_.push_back(host_);
    }

    {
      spdlog::debug("write {}", INDEX);
      std::ofstream ofs(std::filesystem::path(tmp) / INDEX);
      nlohmann::json js(hosts_);
      ofs << std::setw(4) << js << std::endl;
    }
  }

  // create tar file
  {
    spdlog::info("create {}", tar);
    const auto& [status, out, err] =
        palm::shell("/usr/bin/tar",
                    {"cf", tar, "--remove-files", ROOTFS, INDEX, "-C", tmp});
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
  }
  // compress xz file
  if (compress) {
    spdlog::info("create {}", zip);
    const auto& [status, out, err] = palm::shell(
        "/usr/bin/xz",
        {"-z", "-F", "xz", "-C", "sha256", "--best", "-T", "+1", tar});
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
  }

  spdlog::info("done.");
}

static inline std::optional<std::filesystem::path> uncompress(
    const std::string& tar) {
  const auto root = std::format("tmp-{}", palm::timestamp());

  {
    spdlog::debug("create {}", root);
    std::filesystem::create_directory(root);
  }

  {
    spdlog::info("decompress {}", tar);
    const auto& [status, out, err] =
        palm::shell("/usr/bin/tar", {"xf", tar, "-C", root});
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return std::nullopt;
    }
  }
  return root;
}

inline static void upload(std::shared_ptr<palm::minio::Client> client,
                          const std::filesystem::path& rootfs,
                          const std::string& bucket,
                          const std::string& object) {
  if (!client->bucket_exists(bucket)) {
    client->create_bucket(bucket);
  }
  const auto file = rootfs / bucket / object;
  client->upload(bucket, object, file.string());
}

void aloe::s3::restore(const toml::table& config, const std::string& host,
                       const std::string& tar) {
  auto client = palm::minio::Client::open(host);
  client->list_buckets();
  spdlog::info("restore s3 {} to {}", tar, host);
  const auto tmp = uncompress(tar);
  if (!tmp) {
    return;
  }
  spdlog::debug("load file list from {}", INDEX);
  std::ifstream fs(tmp.value() / INDEX);
  auto js = nlohmann::json::parse(fs);
  auto hosts = js.template get<std::vector<Host>>();
  for (const auto& h : hosts) {
    for (const auto& b : h.buckets) {
      for (const auto& o : b.objects) {
        upload(client, tmp.value() / ROOTFS, b.name, o.name);
      }
    }
  }

  spdlog::info("done.");
}

void aloe::s3::restore(const toml::table& config, const std::string& host,
                       const std::string& tar, const std::string& list) {
  auto client = palm::minio::Client::open(host);
  client->list_buckets();
  spdlog::info("restore s3 {} to {} by file list {}", tar, host, list);
  const auto tmp = uncompress(tar);
  if (!tmp) {
    return;
  }
  std::ifstream fs(tmp.value() / INDEX);
  auto js = nlohmann::json::parse(fs);
  auto files = js.template get<std::vector<File>>();
  for (const auto& f : files) {
    upload(client, tmp.value() / ROOTFS, f.bucket, f.object);
  }
  spdlog::info("done.");
}
