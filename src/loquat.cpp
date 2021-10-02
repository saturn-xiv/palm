#include "palm/loquat.hpp"
#include "palm/utils.hpp"

#include <Poco/Util/IniFileConfiguration.h>
#include <yaml-cpp/yaml.h>

static const std::string EXT = ".toml";
static const std::string ALL = "all" + EXT;

// https://marzer.github.io/tomlplusplus/classtoml_1_1table.html#type-checks
static palm::loquat::Value load_toml_node(const toml::node& node) {
  if (node.is_boolean()) {
    const auto it = node.value<bool>();
    return it.value();
  }
  if (node.is_integer()) {
    const auto it = node.value<int>();
    return it.value();
  }
  if (node.is_floating_point()) {
    const auto it = node.value<float>();
    return it.value();
  }
  if (node.is_string()) {
    const auto it = node.value<std::string>();
    return it.value();
  }

  std::stringstream ss;
  ss << "unknown node type " << node.type();
  throw Poco::RuntimeException(ss.str());
}

static palm::loquat::Value load_toml_array(const toml::array& node) {
  const auto it = node.get(0);
  if (it) {
    if (it->is_integer()) {
      std::vector<int> items;
      for (const auto& v : node) {
        const auto it = v.value<int>();
        items.push_back(it.value());
      }
      return items;
    }
    if (it->is_floating_point()) {
      std::vector<float> items{};
      for (const auto& v : node) {
        const auto it = v.value<float>();
        items.push_back(it.value());
      }
      return items;
    }
    if (it->is_string()) {
      std::vector<std::string> items{};
      for (const auto& v : node) {
        const auto it = v.value<std::string>();
        items.push_back(it.value());
      }
      return items;
    }
    std::stringstream ss;
    ss << "unknown node type " << it->type();
    throw Poco::RuntimeException(ss.str());
  }

  std::vector<std::string> items;
  return items;
}

palm::loquat::Host::Host(const std::filesystem::path& file,
                         const palm::loquat::Env& env,
                         std::shared_ptr<Poco::LogStream> logger)
    : env({}), logger(logger) {
  this->name = palm::filename_without_extension(file);
  logger->information() << "find host " << name << std::endl;
  {
    this->env.insert(env.begin(), env.end());
    const auto node = toml::parse_file(file.string());
    const auto it = palm::loquat::load(node);
    this->env.insert(it.begin(), it.end());
  }
}

palm::loquat::Group::Group(const std::filesystem::path& file,
                           const palm::loquat::Env& env,
                           std::shared_ptr<Poco::LogStream> logger)
    : env({}), hosts({}), logger(logger) {
  this->name = palm::filename_without_extension(file);
  logger->information() << "find group " << name << std::endl;
  const auto node = toml::parse_file(file.string());

  for (const auto& it : *node["hosts"].as_array()) {
    const auto host = it.value<std::string>();
    this->hosts.push_back(host.value());
  }

  {
    this->env.insert(env.begin(), env.end());
    const auto it = palm::loquat::load(*node["vars"].as_table());
    this->env.insert(it.begin(), it.end());
  }
}

palm::loquat::Inventory::Inventory(const std::filesystem::path& root,
                                   std::shared_ptr<Poco::LogStream> logger)
    : logger(logger), groups({}), hosts({}) {
  logger->information() << "load inventory from " << root << std::endl;
  this->name = palm::filename_without_extension(root);

  Env env;
  {
    env["deploy.timestamp"] = palm::timestamp();
    env["deploy.uuid"] = 1;
  }
  {
    const auto groups = root / "groups";
    const auto all = (groups / ALL);
    logger->debug() << "load " << all << std::endl;
    const auto node = toml::parse_file(all.string());
    auto g_env = palm::loquat::load(node);
    g_env.insert(env.begin(), env.end());
    for (const auto& entry : std::filesystem::directory_iterator(groups)) {
      const auto file = entry.path();
      if (file.filename() != ALL) {
        Group group(file, g_env, logger);
        this->groups.push_back(group);
      }
    }
  }

  {
    const auto hosts = root / "hosts";
    const auto all = (hosts / ALL);
    logger->debug() << "load " << all << std::endl;
    const auto node = toml::parse_file(all.string());
    auto g_env = palm::loquat::load(node);
    g_env.insert(env.begin(), env.end());
    for (const auto& entry : std::filesystem::directory_iterator(hosts)) {
      const auto file = entry.path();
      if (file.filename() != ALL) {
        Host host(file, g_env, logger);
        this->hosts.push_back(host);
      }
    }
  }
}

palm::loquat::Env palm::loquat::load(const toml::table& node) {
  palm::loquat::Env env;
  for (auto [k, v] : node) {
    if (v.is_array()) {
      env[k] = load_toml_array(*v.as_array());
      continue;
    }
    env[k] = load_toml_node(v);
  }
  return env;
}

void palm::loquat::Task::run() {
  // TODO
}

palm::loquat::Job::Job(const std::filesystem::path& name,
                       std::shared_ptr<Poco::LogStream> logger)
    : logger(logger) {}

std::vector<std::shared_ptr<palm::loquat::Task>> palm::loquat::Job::build(
    const Inventory& inventory) {
  std::vector<std::shared_ptr<palm::loquat::Task>> items;
  //   TODO
  return items;
}

void palm::loquat::Application::launch() {
  Poco::Logger& logger = Poco::Logger::root();
  poco_information_f(logger, "deploy %s@%s", this->job, this->inventory);
  auto lstr = std::make_shared<Poco::LogStream>(logger);
  Job job(this->job, lstr);
  Inventory inventory(this->inventory, lstr);
  //   TODO
  poco_information(logger, "done.");
}
void palm::loquat::Application::defineOptions(Poco::Util::OptionSet& options) {
  palm::Application::defineOptions(options);

  options.addOption(Poco::Util::Option("job", "j", "job folder")
                        .required(true)
                        .repeatable(false)
                        .argument("job")
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleJob)));
  options.addOption(Poco::Util::Option("inventory", "i", "inventory folder")
                        .required(true)
                        .repeatable(false)
                        .argument("inventory")
                        .callback(Poco::Util::OptionCallback<Application>(
                            this, &Application::handleInventory)));
}

std::string palm::loquat::Application::description() {
  return "An auto-ops tools inspired by ansible.";
}

void palm::loquat::Application::handleJob(const std::string& name,
                                          const std::string& value) {
  this->job = value;
}

void palm::loquat::Application::handleInventory(const std::string& name,
                                                const std::string& value) {
  this->inventory = value;
}
