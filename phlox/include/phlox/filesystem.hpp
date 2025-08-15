#pragma once

#include "monitoring.grpc.pb.h"
#include "palm/cache.hpp"
#include "palm/jwt.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"
#include "palm/search.hpp"
#include "palm/theme.hpp"

#include <mutex>
#include <thread>

namespace phlox {

namespace monitoring {
void mount(httplib::Server& server, palm::Theme& theme,
           std::shared_ptr<palm::Jwt> jwt,
           std::shared_ptr<palm::opensearch::Client> search);

namespace logging {
class Source {
 public:
  Source();
  virtual void execute(std::shared_ptr<palm::opensearch::Client> search) = 0;

 protected:
  std::string _hostname;
};
class StdinSource final : public Source {
 public:
  StdinSource() : Source() {}
  void execute(std::shared_ptr<palm::opensearch::Client> search) override;

 private:
};
class FilesystemNotify : public Source {
 public:
  FilesystemNotify();
  ~FilesystemNotify();
  void register_(const std::filesystem::path& file);
  void execute(std::shared_ptr<palm::opensearch::Client> search) override;

 private:
  inline std::vector<palm::monitoring::v1::FileSystemLogsResponse_Item> load(
      const std::filesystem::path& p) {
    std::vector<palm::monitoring::v1::FileSystemLogsResponse_Item> items;
    const auto key = std::filesystem::absolute(p);
    auto it = this->_positions.find(key);
    if (it == this->_positions.end()) {
      std::ifstream ss(key);
      ss.seekg(0L, std::ios::end);
      const int pos = ss.tellg();
      spdlog::debug("jump file {} to the end({})", key.string(), pos);
      this->_positions[key] = pos;
      return items;
    }
    std::ifstream ss(key);
    ss.seekg(it->second, std::ios::beg);
    std::string line;
    int pos = it->second;
    while (std::getline(ss, line)) {
      palm::monitoring::v1::FileSystemLogsResponse_Item log;
      log.set_host(this->_hostname);
      log.set_file(p.string());
      log.set_line(line);
      {
        auto at = log.mutable_created_at();
        palm::now(at);
      }
      items.push_back(log);

      pos += line.length() + 1;
    }

    spdlog::debug("read file {} from {} to {}", key.string(), it->second, pos);
    this->_positions[key] = pos;
    return items;
  }
  std::mutex _mutex;
  std::map<int, std::filesystem::path> _targets;
  std::map<std::filesystem::path, int> _positions;
  int _notify_id;
};
}  // namespace logging

namespace health_checkers {
class HealthChecker {
 public:
  virtual std::string name() = 0;
  virtual bool heartbeat() = 0;
  virtual std::chrono::seconds interval() = 0;
};
class PostgreSql final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};
class MySql final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};
class OpenSearch final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};
class Redis final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};
class RabbitMQ final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};

class Http final : public HealthChecker {
 public:
  std::string name() override;
  bool heartbeat() override;
  std::chrono::seconds interval() override;

 private:
};

}  // namespace health_checkers

class LoggingScratcher {
 public:
  void launch(std::shared_ptr<palm::opensearch::Client> search,
              std::chrono::seconds sleep = std::chrono::seconds{1});
  inline void register_(
      std::shared_ptr<phlox::monitoring::logging::Source> item) {
    std::lock_guard<std::mutex> lock(this->_mutex);
    this->_nodes.push_back(item);
  }

 private:
  std::vector<std::shared_ptr<phlox::monitoring::logging::Source>> _nodes;
  std::mutex _mutex;
};
class HealthCheckWorker {
 public:
  void launch(std::chrono::seconds sleep =
                  std::chrono::duration_cast<std::chrono::seconds>(
                      std::chrono::minutes{1}));
  inline void register_(
      std::shared_ptr<phlox::monitoring::health_checkers::HealthChecker> item) {
    std::lock_guard<std::mutex> lock(this->_mutex);
    this->_nodes.push_back(item);
  }

 private:
  std::vector<
      std::shared_ptr<phlox::monitoring::health_checkers::HealthChecker>>
      _nodes;
  std::mutex _mutex;
};
}  // namespace monitoring
}  // namespace phlox
