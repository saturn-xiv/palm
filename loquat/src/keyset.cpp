#include "loquat/env.hpp"

#include <tink/binary_keyset_reader.h>
#include <tink/binary_keyset_writer.h>
#include <tink/cleartext_keyset_handle.h>

std::unique_ptr<crypto::tink::KeysetHandle> loquat::Keyset::load(
    const google::crypto::tink::KeyTemplate& tpl) {
  const std::lock_guard<std::mutex> lock(this->_locker);

  const auto file = this->keyset();
  if (std::filesystem::exists(file)) {
    spdlog::debug("load keyset from {}", file.string());

    if (std::filesystem::status(file).permissions() !=
        std::filesystem::perms::owner_read) {
      throw std::invalid_argument("key file too open");
    }

    std::unique_ptr<std::ifstream> in =
        std::make_unique<std::ifstream>(file, std::ios_base::binary);
    auto reader_r = crypto::tink::BinaryKeysetReader::New(std::move(in));
    {
      const auto status = reader_r.status();
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return nullptr;
      }
    }
    auto reader = std::move(reader_r.value());
    auto keyset_handle_r =
        crypto::tink::CleartextKeysetHandle::Read(std::move(reader));
    {
      const auto status = keyset_handle_r.status();
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return nullptr;
      }
    }
    auto keyset_handle = std::move(keyset_handle_r.value());
    return keyset_handle;

  } else {
    spdlog::warn("not exists, try to create {}", file.string());
    auto keyset_handle_r = crypto::tink::KeysetHandle::GenerateNew(
        tpl, crypto::tink::KeyGenConfigGlobalRegistry());
    {
      const auto status = keyset_handle_r.status();
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return nullptr;
      }
    }
    auto keyset_handler = std::move(keyset_handle_r.value());
    {
      std::unique_ptr<std::ofstream> out = std::make_unique<std::ofstream>();
      out->open(file, std::ios_base::binary);
      auto writer_r = crypto::tink::BinaryKeysetWriter::New(std::move(out));
      {
        const auto status = writer_r.status();
        if (!status.ok()) {
          spdlog::error("{}", status.message());
          return nullptr;
        }
      }
      auto writer = std::move(writer_r.value());
      const auto status = crypto::tink::CleartextKeysetHandle::Write(
          writer.get(), *keyset_handler.get());
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return nullptr;
      }
    }
    std::filesystem::permissions(file, std::filesystem::perms::owner_read);
    return keyset_handler;
  }
}
