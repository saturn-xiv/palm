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

 private:
  std::string name;
  std::vector<std::string> hosts;
  Env env;
  std::shared_ptr<Poco::LogStream> logger;
};

class Inventory {
 public:
  Inventory() {}
  Inventory(const std::filesystem::path& root,
            std::shared_ptr<Poco::LogStream> logger);

  friend std::ostream& operator<<(std::ostream& out, const Inventory& self) {
    out << "=== Begin Inventory " << self.name << " ===";
    // TODO
    out << "=== End Inventory " << self.name << " ===";
    return out;
  }

 private:
  Env env;
  std::string name;
  std::vector<Group> groups;
  std::vector<Host> hosts;
  std::shared_ptr<Poco::LogStream> logger;
};

class Task {
  Task(const std::string& name, const std::string& script, const Env& env,
       std::shared_ptr<Poco::LogStream> logger)
      : name(name), script(script), env(env), logger(logger) {}
  void run();

 private:
  std::string name;
  std::string script;
  Env env;
  std::shared_ptr<Poco::LogStream> logger;
};

class Job {
 public:
  Job(const std::filesystem::path& root,
      std::shared_ptr<Poco::LogStream> logger);
  std::vector<std::shared_ptr<Task>> build(const Inventory& inventory);
  friend std::ostream& operator<<(std::ostream& out, const Job& self) {
    out << "=== Begin Job " << self.name << " ===";
    // TODO
    out << "=== End Job " << self.name << " ===";

    return out;
  }

 private:
  std::string name;
  Env env;
  std::map<std::string, std::string> tasks;
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
