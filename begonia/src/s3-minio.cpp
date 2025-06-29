#include "palm/crypto.hpp"
#include "palm/s3.hpp"

#include <filesystem>
#include <format>

#include <boost/log/trivial.hpp>

#include <inja/inja.hpp>

#define PALM_OPEN_MINIO_CLIENT(x)                                          \
  BOOST_LOG_TRIVIAL(debug) << "connect https://" << x->_base_url;          \
  ::minio::s3::BaseUrl base_url(x->_base_url);                             \
  ::minio::creds::StaticProvider provider(x->_access_key, x->_secret_key); \
  ::minio::s3::Client client(base_url, &provider)

std::vector<std::string> palm::Minio::list_buckets() {
  PALM_OPEN_MINIO_CLIENT(this);
  const auto res = client.ListBuckets();
  if (!res) {
    BOOST_LOG_TRIVIAL(error) << res.Error();
    return {};
  }

  std::vector<std::string> items;
  for (const auto& bucket : res.buckets) {
    items.push_back(bucket.name);
  }
  return items;
}

bool palm::Minio::bucket_exists(const std::string& name) {
  PALM_OPEN_MINIO_CLIENT(this);
  ::minio::s3::BucketExistsArgs args;
  args.bucket = name;

  const auto res = client.BucketExists(args);
  if (!res) {
    BOOST_LOG_TRIVIAL(error) << res.Error();
    return false;
  }
  return res.exist;
}
void palm::Minio::create_bucket(const std::string& name, bool is_public,
                                std::optional<uint> expiration_days) {
  PALM_OPEN_MINIO_CLIENT(this);
  // https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
  {
    BOOST_LOG_TRIVIAL(info) << "create bucket " << name;
    ::minio::s3::MakeBucketArgs args;
    args.bucket = name;

    ::minio::s3::MakeBucketResponse res = client.MakeBucket(args);
    if (!res) {
      BOOST_LOG_TRIVIAL(error) << res.Error();
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
    BOOST_LOG_TRIVIAL(info) << "set bucket public access";
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
    BOOST_LOG_TRIVIAL(debug) << args.policy;
    ::minio::s3::SetBucketPolicyResponse res = client.SetBucketPolicy(args);
    if (!res) {
      BOOST_LOG_TRIVIAL(error) << res.Error();
      return;
    }
  }

  if (expiration_days && expiration_days.value() > 0) {
    BOOST_LOG_TRIVIAL(info) << "set expires in " << expiration_days.value()
                            << " days for bucket " << name;
    ::minio::s3::LifecycleConfig config;

    {
      ::minio::s3::LifecycleRule rule;
      rule.id = std::format("expires-in-{}-days", expiration_days.value());
      rule.status = true;
      rule.expiration_days = minio::s3::Integer(expiration_days.value());
      rule.filter.prefix = minio::s3::Prefix(std::format("{}/", name));
      config.rules.push_back(rule);
    }

    minio::s3::SetBucketLifecycleArgs args(config);
    args.bucket = name;

    ::minio::s3::SetBucketLifecycleResponse res =
        client.SetBucketLifecycle(args);
    if (!res) {
      BOOST_LOG_TRIVIAL(error) << res.Error();
      return;
    }
  }
}
std::optional<std::string> palm::Minio::upload(const std::string& bucket,
                                               const std::string& filename) {
  PALM_OPEN_MINIO_CLIENT(this);
  std::filesystem::path file(filename);
  const std::string object =
      std::format("{}{}", palm::uuid(), file.extension().string());
  BOOST_LOG_TRIVIAL(info) << "upload " << filename << " to (" << bucket << ","
                          << object << ")";
  ::minio::s3::UploadObjectArgs args;

  args.bucket = bucket;
  args.object = object;
  args.filename = filename;

  ::minio::s3::UploadObjectResponse res = client.UploadObject(args);
  if (!res) {
    BOOST_LOG_TRIVIAL(error) << res.Error();
    return nullptr;
  }
  return object;
}
std::optional<std::string> palm::Minio::get_presigned_object_url(
    const std::string& bucket, const std::string& object,
    const std::string& title, const std::string& content_type,
    const std::chrono::seconds ttl) {
  if (ttl < std::chrono::minutes(1) || ttl > std::chrono::days(7)) {
    BOOST_LOG_TRIVIAL(error) << "bad ttl " << ttl;
    return std::nullopt;
  }
  PALM_OPEN_MINIO_CLIENT(this);

  // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
  // https://min.io/docs/minio/linux/developers/go/API.html#presignedgetobject-ctx-context-context-bucketname-objectname-string-expiry-time-duration-reqparams-url-values-url-url-error
  ::minio::s3::GetPresignedObjectUrlArgs args;
  args.bucket = bucket;
  args.object = object;
  args.method = minio::http::Method::kGet;
  args.expiry_seconds = ttl.count();
  ::minio::utils::Multimap headers;
  headers.Add(
      "response-content-disposition",
      (content_type.starts_with("image/") ||
       content_type.starts_with("video/") || content_type == "application/pdf")
          ? "inline"
          : std::format("attachment; filename=\"{}\"", title));

  args.extra_headers = headers;
  ::minio::s3::GetPresignedObjectUrlResponse res =
      client.GetPresignedObjectUrl(args);
  if (!res) {
    BOOST_LOG_TRIVIAL(error) << res.Error();
    return nullptr;
  }
  return res.url;
}

std::string palm::Minio::get_object(const std::string& bucket,
                                    const std::string& object) {
  return std::format("https://{}/{}/{}", this->_base_url, bucket, object);
}
