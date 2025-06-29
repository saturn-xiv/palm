#pragma once

#include "palm/cache.hpp"
#include "palm/jwt.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"
#include "palm/search.hpp"
#include "palm/theme.hpp"

#include <mutex>
#include <thread>

namespace palm {

namespace monitor {
void mount(httplib::Server& server, palm::Theme& theme,
           std::shared_ptr<palm::Jwt> jwt,
           std::shared_ptr<palm::opensearch::Client> search);

namespace logging {
struct Item {
  static nlohmann::json properties() {
    nlohmann::json props;
    {
      nlohmann::json it;
      it["type"] = "keyword";
      props["host"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "keyword";
      props["file"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "text";
      props["message"] = it;
    }
    {
      nlohmann::json it;
      it["type"] = "unsigned_long";
      props["created_at"] = it;
    }
    return props;
  }
  static inline uint64_t now() {
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
               std::chrono::system_clock::now().time_since_epoch())
        .count();
  }

  std::string host;
  std::string file;
  std::string message;
  uint64_t created_at;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Item, host, file, message, created_at);
};
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
  inline std::vector<std::tuple<std::filesystem::path, std::string, uint64_t>>
  load(const std::filesystem::path& p) {
    std::vector<std::tuple<std::filesystem::path, std::string, uint64_t>> items;
    const auto key = std::filesystem::absolute(p);
    auto it = this->_positions.find(key);
    if (it == this->_positions.end()) {
      std::ifstream ss(key);
      ss.seekg(0L, std::ios::end);
      const int pos = ss.tellg();
      BOOST_LOG_TRIVIAL(debug)
          << "jump file " << key.string() << " to the end(" << pos << ")";
      this->_positions[key] = pos;
      return items;
    }
    std::ifstream ss(key);
    ss.seekg(it->second, std::ios::beg);
    std::string line;
    int pos = it->second;
    while (std::getline(ss, line)) {
      items.push_back({p, line, palm::monitor::logging::Item::now()});
      pos += line.length() + 1;
    }

    BOOST_LOG_TRIVIAL(debug) << "read file " << key.string() << " from "
                             << it->second << " to " << pos;
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
  inline void register_(std::shared_ptr<palm::monitor::logging::Source> item) {
    std::lock_guard<std::mutex> lock(this->_mutex);
    this->_nodes.push_back(item);
  }

 private:
  std::vector<std::shared_ptr<palm::monitor::logging::Source>> _nodes;
  std::mutex _mutex;
};
class HealthCheckWorker {
 public:
  void launch(std::chrono::seconds sleep =
                  std::chrono::duration_cast<std::chrono::seconds>(
                      std::chrono::minutes{1}));
  inline void register_(
      std::shared_ptr<palm::monitor::health_checkers::HealthChecker> item) {
    std::lock_guard<std::mutex> lock(this->_mutex);
    this->_nodes.push_back(item);
  }

 private:
  std::vector<std::shared_ptr<palm::monitor::health_checkers::HealthChecker>>
      _nodes;
  std::mutex _mutex;
};
}  // namespace monitor
}  // namespace palm
