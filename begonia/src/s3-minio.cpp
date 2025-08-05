#include "palm/crypto.hpp"
#include "palm/s3.hpp"

#include <filesystem>
#include <format>

#include <spdlog/spdlog.h>
#include <inja/inja.hpp>

#define PALM_OPEN_MINIO_CLIENT(x)                                          \
  spdlog::debug("connect {}", x->base_url());                              \
  ::minio::s3::BaseUrl base_url(x->_host);                                 \
  base_url.https = x->_https;                                              \
  if (x->_port) {                                                          \
    base_url.port = x->_port.value();                                      \
  }                                                                        \
  ::minio::creds::StaticProvider provider(x->_access_key, x->_secret_key); \
  ::minio::s3::Client client(base_url, &provider)

std::vector<std::string> palm::minio::Client::list_buckets() {
  PALM_OPEN_MINIO_CLIENT(this);
  const auto res = client.ListBuckets();
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return {};
  }

  std::vector<std::string> items;
  for (const auto& bucket : res.buckets) {
    items.push_back(bucket.name);
  }
  return items;
}
std::vector<std::tuple<std::string, size_t>> palm::minio::Client::list_objects(
    const std::string& bucket) {
  PALM_OPEN_MINIO_CLIENT(this);
  ::minio::s3::ListObjectsArgs args;
  args.bucket = bucket;
  auto res = client.ListObjects(args);
  std::vector<std::tuple<std::string, size_t>> items;
  for (; res; res++) {
    ::minio::s3::Item it = *res;
    if (!it) {
      spdlog::error("{}", it.Error().String());
      continue;
    }
    items.push_back({it.name, it.size});
  }
  return items;
}
bool palm::minio::Client::bucket_exists(const std::string& name) {
  PALM_OPEN_MINIO_CLIENT(this);
  ::minio::s3::BucketExistsArgs args;
  args.bucket = name;

  const auto res = client.BucketExists(args);
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return false;
  }
  return res.exist;
}
void palm::minio::Client::create_bucket(const std::string& name, bool is_public,
                                        std::optional<uint> expiration_days) {
  PALM_OPEN_MINIO_CLIENT(this);
  // https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
  {
    spdlog::info("create bucket {}", name);
    ::minio::s3::MakeBucketArgs args;
    args.bucket = name;

    ::minio::s3::MakeBucketResponse res = client.MakeBucket(args);
    if (!res) {
      spdlog::error("{}", res.Error().String());
      return;
    }
  }
  // https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html
  // mc set my-minio https://base-url access-key secret-key
  // mc ls my-minio
  // mc mb my-minio/downloads
  // mc anonymous set download my-minio/downloads
  // mc anonymous get-json my-minio/downloads > /tmp/policy.json
  // mc rb my-minio/downloads
  if (is_public) {
    spdlog::info("set bucket public access");
    const std::string POLICY = R"(
{
  "Statement": [
    {
      "Action": ["s3:GetBucketLocation", "s3:ListBucket"],
      "Effect": "Allow",
      "Principal": { "AWS": ["*"] },
      "Resource": ["arn:aws:s3:::{{ name }}"]
    },
    {
      "Action": ["s3:GetObject"],
      "Effect": "Allow",
      "Principal": { "AWS": ["*"] },
      "Resource": ["arn:aws:s3:::{{ name }}/*"]
    }
  ],
  "Version": "2012-10-17"
}
)";
    nlohmann::json data;
    data["name"] = name;

    ::minio::s3::SetBucketPolicyArgs args;
    args.bucket = name;
    args.policy = inja::render(POLICY, data);
    spdlog::debug(args.policy);
    ::minio::s3::SetBucketPolicyResponse res = client.SetBucketPolicy(args);
    if (!res) {
      spdlog::error("{}", res.Error().String());
      return;
    }
  }

  if (expiration_days && expiration_days.value() > 0) {
    spdlog::info("set expires in {} days for bucket {}",
                 expiration_days.value(), name);
    ::minio::s3::LifecycleConfig config;

    {
      ::minio::s3::LifecycleRule rule;
      rule.id = std::format("expires-in-{}-days", expiration_days.value());
      rule.status = true;
      rule.expiration_days = ::minio::s3::Integer(expiration_days.value());
      rule.filter.prefix = ::minio::s3::Prefix(std::format("{}/", name));
      config.rules.push_back(rule);
    }

    ::minio::s3::SetBucketLifecycleArgs args(config);
    args.bucket = name;

    ::minio::s3::SetBucketLifecycleResponse res =
        client.SetBucketLifecycle(args);
    if (!res) {
      spdlog::error("{}", res.Error().String());
      return;
    }
  }
}
bool palm::minio::Client::upload(const std::string& bucket,
                                 const std::string& object,
                                 const std::string& filename) {
  PALM_OPEN_MINIO_CLIENT(this);
  spdlog::info("upload {} to ({}, {})", filename, bucket, object);
  ::minio::s3::UploadObjectArgs args;

  args.bucket = bucket;
  args.object = object;
  args.filename = filename;

  ::minio::s3::UploadObjectResponse res = client.UploadObject(args);
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return false;
  }
  return true;
}
std::optional<std::string> palm::minio::Client::get_presigned_object_url(
    const std::string& bucket, const std::string& object,
    const std::string& title, const std::string& content_type,
    const std::chrono::seconds ttl) {
  if (ttl < std::chrono::minutes(1) || ttl > std::chrono::days(7)) {
    spdlog::error("bad ttl {}", ttl.count());
    return std::nullopt;
  }
  PALM_OPEN_MINIO_CLIENT(this);

  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
  // https://min.io/docs/minio/linux/developers/go/API.html#presignedgetobject-ctx-context-context-bucketname-objectname-string-expiry-time-duration-reqparams-url-values-url-url-error
  ::minio::s3::GetPresignedObjectUrlArgs args;
  args.bucket = bucket;
  args.object = object;
  args.method = ::minio::http::Method::kGet;
  args.expiry_seconds = ttl.count();
  ::minio::utils::Multimap headers;
  headers.Add(
      "response-content-disposition",
      (content_type.starts_with("image/") ||
       content_type.starts_with("video/") || content_type == "application/pdf")
          ? "inline"
          : std::format("attachment; filename=\"{}\"", title));

  args.extra_headers = headers;
  spdlog::debug("get presigned object url ({}, {})", bucket, object);
  ::minio::s3::GetPresignedObjectUrlResponse res =
      client.GetPresignedObjectUrl(args);
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return nullptr;
  }
  return res.url;
}

std::string palm::minio::Client::get_object(const std::string& bucket,
                                            const std::string& object) {
  return std::format("{}/{}/{}", this->base_url(), bucket, object);
}

bool palm::minio::Client::get_object(const std::string& bucket,
                                     const std::string& object,
                                     std::ofstream& output) {
  PALM_OPEN_MINIO_CLIENT(this);
  ::minio::s3::GetObjectArgs args;
  args.bucket = bucket;
  args.object = object;
  args.datafunc = [&](::minio::http::DataFunctionArgs args) -> bool {
    if (!output.is_open()) {
      spdlog::error("file was closed");
      return false;
    }
    output << args.datachunk;
    return true;
  };
  spdlog::debug("get object ({}, {})", bucket, object);
  ::minio::s3::GetObjectResponse res = client.GetObject(args);
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return false;
  }
  return true;
}

std::optional<std::tuple<std::string, std::string, size_t, std::string>>
palm::minio::Client::stat_object(const std::string& bucket,
                                 const std::string& object) {
  PALM_OPEN_MINIO_CLIENT(this);
  ::minio::s3::StatObjectArgs args;
  args.bucket = bucket;
  args.object = object;
  spdlog::debug("stat object ({}, {})", bucket, object);
  ::minio::s3::StatObjectResponse res = client.StatObject(args);
  if (!res) {
    spdlog::error("{}", res.Error().String());
    return std::nullopt;
  }
  std::tuple<std::string, std::string, size_t, std::string> it = {
      res.version_id, res.etag, res.size, res.last_modified.ToISO8601UTC()};
  return it;
}
