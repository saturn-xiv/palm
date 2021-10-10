#include "palm/loquat.hpp"
#include "palm/crypto.hpp"
#include "palm/utils.hpp"

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
    this->hosts.insert(host.value());
  }

  {
    this->env.insert(env.begin(), env.end());
    const auto it = palm::loquat::load(*node["vars"].as_table());
    this->env.insert(it.begin(), it.end());
  }
}

std::vector<palm::loquat::Env> palm::loquat::Inventory::by_group(
    const std::string& group) const {
  std::vector<palm::loquat::Env> items;
  for (const auto& ig : this->groups) {
    if (ig.name == group) {
      for (const auto& ih : ig.hosts) {
        palm::loquat::Env it;
        it.insert(ig.env.begin(), ig.env.end());
        const auto env = this->by_host(ih);
        it.insert(env.begin(), env.end());
        items.push_back(it);
      }
    }
  }

  return items;
}

palm::loquat::Env palm::loquat::Inventory::by_host(
    const std::string& host) const {
  palm::loquat::Env env;
  for (const auto& it : this->hosts) {
    if (host == it.name) {
      env.insert(it.env.begin(), it.env.end());
    }
  }

  const auto pos = env.find(Executor::SSH_HOST);
  if (pos == env.end()) {
    env[Executor::SSH_HOST] = host;
  }
  return env;
}

palm::loquat::Inventory::Inventory(const std::filesystem::path& root,
                                   std::shared_ptr<Poco::LogStream> logger)
    : logger(logger), groups({}), hosts({}) {
  logger->information() << "load inventory from " << root << std::endl;
  this->name = palm::filename_without_extension(root);

  Env env;
  {
    env["deploy.timestamp"] = palm::timestamp();
    env["deploy.uuid"] = palm::uuid::v4();
    env["deploy.user"] = palm::current_user();
    env["deploy.hostname"] = std::string(palm::os().nodename);
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

palm::loquat::Executor::Executor(const std::string& role, const Env& env)
    : role(role), env(env) {
  {
    const auto pos = this->env.find(SSH_HOST);
    // if (pos == this->env.end()) {
    //   throw Poco::RuntimeException("can't find ssh host");
    // }
    this->host = std::get<std::string>(pos->second);
  }
  if (this->env.find(SSH_PORT) == this->env.end()) {
    this->env[SSH_PORT] = 22;
  }
  if (this->env.find(SSH_USER) == this->env.end()) {
    this->env[SSH_USER] = std::string("root");
  }

  if (this->env.find(SSH_IDENTITY_FILE) == this->env.end()) {
    // .ssh/id_rsa
    this->env[SSH_IDENTITY_FILE] = std::string(".ssh/id_ed25519");
  }
}

std::vector<palm::loquat::Executor> palm::loquat::Task::executors(
    const palm::loquat::Inventory& inventory) const {
  std::vector<Executor> items;

  for (const auto& role : this->roles) {
    for (const auto& group : this->groups) {
      for (const auto& env : inventory.by_group(group)) {
        Executor it(role, env);
        items.push_back(it);
      }
    }
    for (const auto& host : this->hosts) {
      const auto env = inventory.by_host(host);
      Executor it(role, env);
      items.push_back(it);
    }
  }

  return items;
}

palm::loquat::Job::Job(const std::filesystem::path& file,
                       std::shared_ptr<Poco::LogStream> logger)
    : logger(logger), tasks({}) {
  logger->information() << "load job from " << file;
  const auto node = toml::parse_file(file.string());
  this->name = palm::filename_without_extension(file);
  for (auto [k, v] : node) {
    Task task;
    task.name = k;
    task.logger = this->logger;
    auto node = v.as_table();
    for (const auto& it : *(*node)["hosts"].as_array()) {
      const auto h = it.value<std::string>();
      task.hosts.push_back(h.value());
    }
    for (const auto& it : *(*node)["groups"].as_array()) {
      const auto g = it.value<std::string>();
      task.groups.push_back(g.value());
    }
    for (const auto& it : *(*node)["roles"].as_array()) {
      const auto r = it.value<std::string>();
      task.roles.push_back(r.value());
    }
    this->tasks.push_back(task);
  }
}

void palm::loquat::Job::execute(
    const palm::loquat::Inventory& inventory) const {
  std::vector<std::vector<palm::loquat::Executor>> items;
  for (const auto& task : this->tasks) {
    for (const auto& role : task.roles) {
      const auto executors = task.executors(inventory);
      // TODO run
      items.push_back(executors);
    }
  }
}

int palm::loquat::Application::launch() {
  Poco::Logger& logger = Poco::Logger::root();
  poco_information_f(logger, "deploy %s@%s", this->job, this->inventory);
  auto lstr = std::make_shared<Poco::LogStream>(logger);

  try {
    const Job job(this->job, lstr);
    const Inventory inventory(this->inventory, lstr);
    job.execute(inventory);
    poco_information(logger, "done.");
    return Poco::Util::Application::EXIT_OK;
  } catch (Poco::Exception& ex) {
    lstr->error() << ex.displayText() << std::endl;
    return Poco::Util::Application::EXIT_SOFTWARE;
  }
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
