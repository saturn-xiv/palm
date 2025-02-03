#include "marguerite/minio.hpp"
#include "marguerite/utils.hpp"

#include <fstream>

#include <fmt/chrono.h>
#include <cppcodec/base32_hex.hpp>

bool marguerite::Minio::bucket_exist(const std::string& name) {
  minio::s3::BucketExistsArgs args;
  args.bucket = name;

  auto cli = this->open();
  minio::s3::BucketExistsResponse res = cli.BucketExists(args);
  if (!res) {
    throw std::invalid_argument(res.Error().String());
  }
  return res.exist;
}

std::vector<std::string> marguerite::Minio::list_bucket() {
  minio::s3::ListBucketsArgs args;

  auto cli = this->open();
  auto res = cli.ListBuckets(args);
  if (!res) {
    throw std::invalid_argument(res.Error().String());
  }
  std::vector<std::string> items;

  for (const auto& it : res.buckets) {
    items.push_back(it.name);
  }
  return items;
}
std::string marguerite::Minio::get_presigned_object_url(
    const std::string& bucket, const std::string& object,
    const std::chrono::seconds& expiry_seconds) {
  minio::s3::GetPresignedObjectUrlArgs args;
  args.bucket = bucket;
  args.object = object;
  args.method = minio::http::Method::kGet;
  args.expiry_seconds = expiry_seconds.count();

  auto cli = this->open();
  const auto res = cli.GetPresignedObjectUrl(args);
  if (!res) {
    throw std::invalid_argument(res.Error().String());
  }
  return res.url;
}
void marguerite::Minio::remove_object(const std::string& bucket,
                                      const std::string& object) {
  minio::s3::RemoveObjectArgs args;
  args.bucket = bucket;
  args.object = object;

  auto cli = this->open();
  const auto res = cli.RemoveObject(args);
  if (!res) {
    throw std::invalid_argument(res.Error().String());
  }
}
void marguerite::Minio::remove_bucket(const std::string& name) {
  minio::s3::RemoveBucketArgs args;
  args.bucket = name;

  auto cli = this->open();
  const auto res = cli.RemoveBucket(args);
  if (!res) {
    throw std::invalid_argument(res.Error().String());
  }
}

std::pair<std::string, uint64_t> marguerite::Minio::put_object(
    const std::string& bucket, const std::filesystem::path& file) {
  std::string object =
      fmt::format("{}{}", marguerite::uuid(), file.extension().string());
  spdlog::info("upload {} to ({},{})", file.string(), bucket, object);

  uint64_t size = std::filesystem::file_size(file);
  std::ifstream ifs(file);

  minio::s3::PutObjectArgs args(ifs, size, 0);
  args.bucket = bucket;
  args.object = object;

  auto cli = this->open();
  const auto res = cli.PutObject(args);
  if (!res) {
    std::invalid_argument(res.Error().String());
  }

  auto it = std::make_pair(object, size);
  return it;
}

void marguerite::Minio::create_bucket(
    const std::string& name, bool is_public,
    std::optional<std::chrono::days> expiration_days) {
  auto cli = this->open();
  {
    spdlog::warn("create bucket {}", name);
    minio::s3::MakeBucketArgs args;
    args.bucket = name;

    auto cli = this->open();
    const auto res = cli.MakeBucket(args);
    if (!res) {
      std::invalid_argument(res.Error().String());
    }
  }
  if (is_public) {
    spdlog::warn("set public download policy for bucket {}", name);
    minio::s3::SetBucketPolicyArgs args;
    args.bucket = name;
    args.policy = this->build_bucket_policy(name);
    const auto res = cli.SetBucketPolicy(args);
    if (!res) {
      std::invalid_argument(res.Error().String());
    }
  }
  if (expiration_days) {
    spdlog::warn("set bucket lifecycle to {}", expiration_days.value());

    minio::s3::LifecycleConfig config;
    {
      minio::s3::LifecycleRule rule;
      rule.id = "v" + marguerite::timestamp();
      rule.status = true;
      rule.expiration_days = minio::s3::Integer(expiration_days->count());
      config.rules.push_back(rule);
    }

    minio::s3::SetBucketLifecycleArgs args(config);
    args.bucket = name;

    auto const res = cli.SetBucketLifecycle(args);
    if (!res) {
      std::invalid_argument(res.Error().String());
    }
  }
}

// https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html
std::string marguerite::Minio::build_bucket_policy(const std::string& name) {
  nlohmann::json val;
  val["version"] = "2012-10-17";
  val["name"] = name;
  return this->_env.render(R"(
{
  "Version": "{{ version }}",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": { "AWS": "*" },
      "Action": ["s3:GetObject"],
      "Resource": "arn:aws:s3:::{{ name }}/*"
    }
  ]
}
)",
                           val);
}
