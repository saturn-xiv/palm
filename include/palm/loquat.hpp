#pragma once

#include "palm/utils.hpp"

namespace palm {
namespace loquat {

using Value = std::variant<bool, int, float, std::string, std::vector<int>,
                           std::vector<float>, std::vector<std::string>>;

struct value2string {
  std::string operator()(const std::string& x) const { return x; }
  std::string operator()(const int x) const { return std::to_string(x); }
  std::string operator()(const bool x) const {
    std::stringstream ss;
    ss << std::boolalpha;
    ss << x;
    return ss.str();
  }
  std::string operator()(const float x) const {
    std::stringstream ss;
    ss << x;
    return ss.str();
  }
  std::string operator()(const std::vector<int>& x) const {
    std::stringstream ss;
    ss << '[';
    std::ranges::copy(x, std::ostream_iterator<int>(ss, ", "));
    ss << ']';
    return ss.str();
  }
  std::string operator()(const std::vector<float>& x) const {
    std::stringstream ss;
    ss << '[';
    std::ranges::copy(x, std::ostream_iterator<float>(ss, ", "));
    ss << ']';
    return ss.str();
  }
  std::string operator()(const std::vector<std::string>& x) const {
    std::stringstream ss;
    ss << '[';
    std::copy(x.begin(), x.end(), std::ostream_iterator<std::string>(ss, ", "));
    ss << ']';
    return ss.str();
  }
};

struct value2json {
  nlohmann::json operator()(const std::string& x) const { return x; }
  nlohmann::json operator()(const int x) const { return x; }
  nlohmann::json operator()(const bool x) const { return x; }
  nlohmann::json operator()(const float x) const { return x; }
  nlohmann::json operator()(const std::vector<int>& x) const { return x; }
  nlohmann::json operator()(const std::vector<float>& x) const { return x; }
  nlohmann::json operator()(const std::vector<std::string>& x) const {
    return x;
  }
};

using Env = std::map<std::string, Value>;

class Host {
 public:
  Host() {}
  Host(const std::filesystem::path& file, const Env& env,
       std::shared_ptr<Poco::LogStream> logger);

  friend std::ostream& operator<<(std::ostream& out, const Host& self) {
    out << "--- Host " << self.name << " ---\n";
    for (const auto& [k, v] : self.env) {
      out << k << "=" << std::visit(value2string(), v) << "\n";
    }
    out << "------\n";
    return out;
  }
  friend class Inventory;

 private:
  Env env;
  std::string name;
  std::shared_ptr<Poco::LogStream> logger;
};

class Group {
 public:
  Group() {}
  Group(const std::filesystem::path& file, const Env& env,
        std::shared_ptr<Poco::LogStream> logger);
  friend std::ostream& operator<<(std::ostream& out, const Group& self) {
    out << "--- Group " << self.name << " ---\n";
    out << "hosts: ";
    std::copy(self.hosts.begin(), self.hosts.end(),
              std::ostream_iterator<std::string>(out, ", "));
    out << "\n";
    for (const auto& [k, v] : self.env) {
      out << k << "=" << std::visit(value2string(), v) << "\n";
    }
    out << "------\n";
    return out;
  }
  friend class Inventory;

 private:
  std::string name;
  std::set<std::string> hosts;
  Env env;
  std::shared_ptr<Poco::LogStream> logger;
};

class Inventory {
 public:
  Inventory() {}
  Inventory(const std::filesystem::path& root,
            std::shared_ptr<Poco::LogStream> logger);

  std::vector<Env> by_group(const std::string& group) const;
  Env by_host(const std::string& host) const;

  friend std::ostream& operator<<(std::ostream& out, const Inventory& self) {
    out << "=== Inventory " << self.name << " ===\n";
    for (const auto& it : self.groups) {
      out << it;
    }
    for (const auto& it : self.hosts) {
      out << it;
    }
    out << "======\n";
    return out;
  }

 private:
  std::string name;
  std::vector<Group> groups;
  std::vector<Host> hosts;
  std::shared_ptr<Poco::LogStream> logger;
};

class Executor {
 public:
  Executor(const std::string& role, const Env& env);

  inline const static std::string SSH_HOST = "ssh.host";
  inline const static std::string SSH_PORT = "ssh.port";
  inline const static std::string SSH_USER = "ssh.user";
  inline const static std::string SSH_IDENTITY_FILE = "identity-file";

  inline void log(const std::string& message) {
    std::lock_guard<std::mutex> _l(locker);
    const auto root = std::filesystem::path("tmp") / "logs";
    if (!std::filesystem::exists(root)) {
      std::filesystem::create_directories(root);
    }

    auto now = std::chrono::system_clock::now();
    auto itt = std::chrono::system_clock::to_time_t(now);

    std::ofstream file((root / (this->host + ".log")),
                       (std::ios::out | std::ios::app));
    file << std::put_time(std::gmtime(&itt), "%FT%TZ") << message << std::endl;
    file.close();
  }

 private:
  Env env;
  std::string role;
  std::string host;

  static std::mutex locker;
};

class Task {
 public:
  Task() {}

  std::vector<Executor> executors(const Inventory& inventory) const;

  friend class Job;

  friend std::ostream& operator<<(std::ostream& out, const Task& self) {
    out << "--- Task " << self.name << " ---\n";
    out << "groups: ";
    std::copy(self.groups.begin(), self.groups.end(),
              std::ostream_iterator<std::string>(out, ", "));
    out << "\n";
    out << "hosts: ";
    std::copy(self.hosts.begin(), self.hosts.end(),
              std::ostream_iterator<std::string>(out, ", "));
    out << "\n";
    out << "roles: ";
    std::copy(self.roles.begin(), self.roles.end(),
              std::ostream_iterator<std::string>(out, ", "));
    out << "\n";
    out << "------\n";
    return out;
  }

 private:
  std::vector<std::string> groups;
  std::vector<std::string> roles;
  std::vector<std::string> hosts;
  std::string name;
  std::shared_ptr<Poco::LogStream> logger;
};

class Job {
 public:
  Job(const std::filesystem::path& file,
      std::shared_ptr<Poco::LogStream> logger);
  friend std::ostream& operator<<(std::ostream& out, const Job& self) {
    out << "=== Job " << self.name << " ===\n";
    for (const auto& it : self.tasks) {
      out << it;
    }
    out << "======\n";
    return out;
  }

 private:
  std::string name;
  std::vector<Task> tasks;
  std::shared_ptr<Poco::LogStream> logger;
};

Env load(const toml::table& node);

class Application : public palm::Application {
 protected:
  void launch() override;
  void defineOptions(Poco::Util::OptionSet& options) override;
  std::string description() override;

 private:
  std::string job;
  std::string inventory;
  void handleJob(const std::string& name, const std::string& value);
  void handleInventory(const std::string& name, const std::string& value);
};

}  // namespace loquat
}  // namespace palm

namespace nlohmann {

template <>
struct adl_serializer<palm::loquat::Value> {
  static void to_json(nlohmann::json& j, const palm::loquat::Value& opt) {
    j = std::visit(palm::loquat::value2json(), opt);
  }
};
}  // namespace nlohmann
