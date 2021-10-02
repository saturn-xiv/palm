#include "palm/loquat.hpp"
#include "palm/utils.hpp"

#include <Poco/Util/IniFileConfiguration.h>
#include <yaml-cpp/yaml.h>

static const std::string EXT = ".toml";
static const std::string ALL = "all" + EXT;

palm::loquat::Host::Host(const std::filesystem::path& root,
                         const std::string& name,
                         std::shared_ptr<Poco::LogStream> logger)
    : logger(logger) {
  logger->information() << "load host " << name << " from " << root
                        << std::endl;
  // TODO
}

palm::loquat::Group::Group(const std::filesystem::path& root,
                           const std::string& name,
                           std::shared_ptr<Poco::LogStream> logger)
    : logger(logger) {
  // TODO
}

palm::loquat::Inventory::Inventory(const std::filesystem::path& root,
                                   std::shared_ptr<Poco::LogStream> logger)
    : logger(logger) {
  logger->information() << "load inventory from " << root << std::endl;

  {
    const auto groups = root / "groups";
    const auto all = (groups / ALL);
    logger->debug() << "load " << all << std::endl;
    const auto node = toml::parse_file(all.string());
    auto env = palm::loquat::load(node);
    for (const auto& entry : std::filesystem::directory_iterator(groups)) {
      const auto it = entry.path();
      if (it.filename() != ALL) {
        logger->debug() << "find group " << entry.path() << std::endl;
      }
    }
  }

  for (const auto& entry :
       std::filesystem::directory_iterator(root / "hosts")) {
    const auto it = entry.path();
    if (it.filename() == ALL) {
    } else {
      logger->debug() << "find host " << entry.path() << " "
                      << entry.path().filename() << std::endl;
    }
  }
  // {
  //   auto it = root / ALL;
  //   logger->debug() << "read " << it << std::endl;
  //   auto cfg = toml::parse_file(it.string());
  // }
}

// https://marzer.github.io/tomlplusplus/classtoml_1_1table.html#type-checks
static palm::loquat::Value load_toml_node(const toml::node& node) {
  if (node.is_boolean()) {
    return node.value_or<bool>(false);
  }
  if (node.is_integer()) {
    return node.value_or<int>(0);
  }
  if (node.is_floating_point()) {
    return node.value_or<float>(0.0);
  }
  if (node.is_string()) {
    return node.value_or<std::string>("");
  }

  std::stringstream ss;
  ss << "unknown node type " << node.type();
  throw Poco::RuntimeException(ss.str());
}

static palm::loquat::Value load_toml_array(const toml::array& node) {
    const auto it = node.get(0);
  if (it) {
    if (it->is_integer()) {
      std::vector<int> items{};
      for (const auto& v : node) {
        items.push_back(v.value_or<int>(0));
      }
      return items;
    }
    if (it->is_floating_point()) {
      std::vector<float> items{};
      for (const auto& v : node) {
        items.push_back(v.value_or<float>(0.0));
      }
      return items;
    }
    if (it->is_string()) {
      std::vector<std::string> items{};
      for (const auto& v : node) {
        items.push_back(v.value_or<std::string>(""));
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
