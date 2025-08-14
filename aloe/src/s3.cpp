#include "aloe/s3.hpp"
#include "palm/crypto.hpp"
#include "palm/s3.hpp"
#include "palm/theme.hpp"
#include "palm/utils.hpp"

static inline bool download(std::shared_ptr<palm::minio::Client> client,
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
    return false;
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
    return ok;
  }
}

template <typename T>
inline static void save_report(const std::string& name,
                               const std::vector<T>& items) {
  const std::string file =
      std::format("{}-{}-failed.json", name, palm::timestamp());
  spdlog::debug("write {}", file);
  std::ofstream ofs(file);
  nlohmann::json js(items);
  ofs << std::setw(2) << js << std::endl;
}

void aloe::s3::dump(const std::set<std::string>& hosts, bool compress) {
  if (hosts.empty()) {
    spdlog::warn("empty s3 hosts");
    return;
  }
  const auto tmp = palm::timestamp();
  const auto tar = std::format("{}.tar", tmp);
  const auto zip = std::format("{}.tar.xz", tmp);
  const auto md5 = std::format("{}.md5", tmp);

  std::vector<std::tuple<std::string, std::string, std::string>> failed;

  spdlog::info("dump s3 {} to {}", boost::algorithm::join(hosts, ","), tmp);

  std::vector<Host> hosts_;
  size_t cnt = 0;
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
          cnt++;
          spdlog::debug("[{}] found object {}://{}/{}", cnt, cli->base_url(),
                        bucket, name);
          if (download(cli, bucket, name, root)) {
            Object object_ = {.name = name, .size = size};
            bucket_.objects.push_back(object_);
          } else {
            failed.push_back({host, bucket, name});
          }
        }
        host_.buckets.push_back(bucket_);
      }

      hosts_.push_back(host_);
    }

    {
      spdlog::debug("write {}", INDEX);
      std::ofstream ofs(std::filesystem::path(tmp) / INDEX);
      nlohmann::json js(hosts_);
      ofs << std::setw(2) << js << std::endl;
    }
  }

  // create tar file
  {
    spdlog::info("create {}", tar);
    // const auto& [status, out, err] =
    //     palm::shell("/usr/bin/tar",
    //                 {"cf", tar, "-C", tmp, "--remove-files", ROOTFS, INDEX});
    const auto& [status, out, err] = palm::tar(tar, "tmp");
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }

    {
      const auto hash = palm::md5(tar);
      std::ofstream out(md5);
      if (!out.is_open()) {
        spdlog::error("couldn't create md5 file");
        return;
      }
      out << hash.value() << " " << tar << std::endl;
      out.close();
    }
  }
  // compress xz file
  if (compress) {
    spdlog::info("create {}", zip);
    // const auto& [status, out, err] = palm::shell(
    //     "/usr/bin/xz",
    //     {"-z", "-F", "xz", "-C", "sha256", "--best", "-T", "+1", tar});
    const auto& [status, out, err] = palm::xz(tar);
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return;
    }
    {
      const auto hash = palm::md5(zip);
      std::ofstream out(md5, std::ios::app);
      if (!out.is_open()) {
        spdlog::error("couldn't open md5 file");
        return;
      }
      out << hash.value() << " " << zip << std::endl;
      out.close();
    }
  }

  // for (const auto& [h, b, o] : failed) {
  //   spdlog::error("failed to fetch file ({}, {}, {})", h, b, o);
  // }
  if (failed.size() > 0) {
    save_report("dump-failed", failed);
  }
  spdlog::info("done({} failed).", failed.size());
}

static inline std::optional<std::filesystem::path> uncompress(
    const std::string& tar) {
  const auto root = std::format("tmp-{}", palm::timestamp());

  {
    spdlog::info("decompress {}", tar);
    const auto& [status, out, err] = palm::uncompress(tar, root);
    spdlog::debug("{}", out);
    if (status != EXIT_SUCCESS) {
      spdlog::error("{} {}", status, err);
      return std::nullopt;
    }
  }
  return root;
}

inline static bool upload(std::shared_ptr<palm::minio::Client> client,
                          const std::filesystem::path& rootfs,
                          const std::string& bucket,
                          const std::string& object) {
  if (!client->bucket_exists(bucket)) {
    client->create_bucket(bucket);
  }
  const auto file = rootfs / bucket / object;
  return client->upload(bucket, object, file.string());
}

void aloe::s3::restore(const std::string& host, const std::string& tar) {
  auto client = palm::minio::Client::open(host);
  client->list_buckets();
  spdlog::info("restore s3 {} to {}", tar, host);
  const auto tmp = uncompress(tar);
  if (!tmp) {
    return;
  }
  std::vector<File> failed;
  std::vector<File> skipped;
  spdlog::debug("load file list from {}", INDEX);
  std::ifstream fs(tmp.value() / INDEX);
  auto js = nlohmann::json::parse(fs);
  auto hosts = js.template get<std::vector<Host>>();
  size_t cnt = 0;
  for (const auto& h : hosts) {
    for (const auto& b : h.buckets) {
      for (const auto& o : b.objects) {
        cnt++;
        spdlog::debug("[{}] upload {}://{}/{}", cnt, client->base_url(), b.name,
                      o.name);
        if (client->stat_object(b.name, o.name)) {
          spdlog::warn("already exists");
          File it = {.bucket = b.name, .object = o.name};
          skipped.push_back(it);
          continue;
        }
        if (!upload(client, tmp.value() / ROOTFS, b.name, o.name)) {
          File it = {.bucket = b.name, .object = o.name};
          failed.push_back(it);
        }
      }
    }
  }
  if (skipped.size() > 0) {
    save_report("restore-skipped", skipped);
  }
  if (failed.size() > 0) {
    save_report("restore-failed", failed);
  }

  {
    spdlog::info("clean {}", tmp->string());
    std::filesystem::remove_all(tmp.value());
  }
  spdlog::info("done({} skipped, {} failed).", skipped.size(), failed.size());
}

void aloe::s3::restore(const std::string& host, const std::string& tar,
                       const std::string& list) {
  auto client = palm::minio::Client::open(host);
  client->list_buckets();
  spdlog::info("restore s3 {} to {} by file list {}", tar, host, list);
  const auto tmp = uncompress(tar);
  if (!tmp) {
    return;
  }
  std::ifstream fs(list);
  auto js = nlohmann::json::parse(fs);
  auto files = js.template get<std::vector<File>>();
  std::vector<File> failed;
  std::vector<File> skipped;
  size_t cnt = 0;
  for (const auto& f : files) {
    cnt++;
    spdlog::debug("[{}/{}] upload {}://{}/{}", cnt, files.size(),
                  client->base_url(), f.bucket, f.object);
    if (client->stat_object(f.bucket, f.object)) {
      spdlog::warn("already exists");
      File it = {.bucket = f.bucket, .object = f.object};
      skipped.push_back(it);
      continue;
    }
    if (!upload(client, tmp.value() / ROOTFS, f.bucket, f.object)) {
      File it = {.bucket = f.bucket, .object = f.object};
      failed.push_back(it);
    }
  }
  if (skipped.size() > 0) {
    save_report("restore-skipped", skipped);
  }
  if (failed.size() > 0) {
    save_report("restore-failed", failed);
  }

  {
    spdlog::info("clean {}", tmp->string());
    std::filesystem::remove_all(tmp.value());
  }
  spdlog::info("done({}skipped, {} failed).", skipped.size(), failed.size());
}

void aloe::s3::sync(const std::string& source_,
                    const std::string& destination_) {
  spdlog::info("sync s3 from {} to {}", source_, destination_);
  auto source = palm::minio::Client::open(source_);
  auto destination = palm::minio::Client::open(destination_);

  const auto buckets = source->list_buckets();
  spdlog::debug("found {} buckets", buckets.size());
  const std::filesystem::path rootfs = std::format("tmp-{}", palm::timestamp());
  std::vector<File> failed;
  std::vector<File> skipped;
  size_t cnt = 0;
  for (auto const& bucket : buckets) {
    spdlog::debug("fetch bucket {}", bucket);
    const auto objects = source->list_objects(bucket);
    for (auto const& [name, size] : objects) {
      cnt++;
      spdlog::info("[{}] fetch {}://{}/{} {} bytes", cnt, source->base_url(),
                   bucket, name, size);
      if (destination->stat_object(bucket, name)) {
        spdlog::warn("file {}://{}/{} already exists", destination->base_url(),
                     bucket, name);
        File it = {.bucket = bucket, .object = name};
        skipped.push_back(it);
        continue;
      }
      if (download(source, bucket, name, rootfs)) {
        if (upload(destination, rootfs, bucket, name)) {
          continue;
        }
      }
      File it = {.bucket = bucket, .object = name};
      failed.push_back(it);
    }
  }
  {
    spdlog::debug("clean {}", rootfs.string());
    std::filesystem::remove_all(rootfs);
  }
  if (skipped.size() > 0) {
    save_report("sync-skipped", skipped);
  }
  if (failed.size() > 0) {
    save_report("sync-failed", failed);
  }
  spdlog::info("done({} failed).", failed.size());
}
void aloe::s3::sync(const std::string& source_, const std::string& destination_,
                    const std::string& file_list_) {
  spdlog::info("sync s3 from {} to {} by files list {}", source_, destination_,
               file_list_);
  std::ifstream fs(file_list_);
  auto js = nlohmann::json::parse(fs);
  auto files = js.template get<std::vector<File>>();

  auto source = palm::minio::Client::open(source_);
  auto destination = palm::minio::Client::open(destination_);
  const std::filesystem::path rootfs = std::format("tmp-{}", palm::timestamp());
  std::vector<File> failed;
  std::vector<File> skipped;
  size_t cnt = 0;
  for (auto const& file : files) {
    cnt++;
    spdlog::info("[{}/{}] fetch {}/{} ", cnt, files.size(), file.bucket,
                 file.object);
    if (!source->stat_object(file.bucket, file.object)) {
      spdlog::warn("couldn't found {}://{}/{}", source->base_url(), file.bucket,
                   file.object);
      File it = {.bucket = file.bucket, .object = file.object};
      skipped.push_back(it);
      continue;
    }
    if (destination->stat_object(file.bucket, file.object)) {
      spdlog::warn("file {}://{}/{} already exists", destination->base_url(),
                   file.bucket, file.object);
      File it = {.bucket = file.bucket, .object = file.object};
      skipped.push_back(it);
      continue;
    }
    if (download(source, file.bucket, file.object, rootfs)) {
      if (upload(destination, rootfs, file.bucket, file.object)) {
        continue;
      }
    }
    File it = {.bucket = file.bucket, .object = file.object};
    failed.push_back(it);
  }

  {
    spdlog::info("clean {}", rootfs.string());
    std::filesystem::remove_all(rootfs);
  }
  if (skipped.size() > 0) {
    save_report("sync-skipped", skipped);
  }
  if (failed.size() > 0) {
    save_report("sync-failed", failed);
  }
  spdlog::info("done({} skipped, {} failed).", skipped.size(), failed.size());
}
