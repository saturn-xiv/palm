#include "iris/application.hpp"
#include "iris/database.hpp"
#include "iris/filesystem.hpp"
#include "iris/minio.hpp"
#include "iris/utils.hpp"
#include "iris/version.hpp"

#include <algorithm>
#include <cstdlib>

#include <spdlog/spdlog.h>
#include <argparse/argparse.hpp>

static inline void keep_files(const std::string& prefix,
                              const std::string& suffix, size_t count) {
  std::vector<std::string> items;
  for (const auto& entry :
       std::filesystem::directory_iterator(std::filesystem::current_path())) {
    const auto path = entry.path();
    if (!std::filesystem::is_regular_file(path)) {
      continue;
    }
    {
      const auto name = path.filename().string();
      if (!name.starts_with(prefix)) {
        continue;
      }
      if (!name.ends_with(suffix)) {
        continue;
      }
    }
    const auto file = path.string();
    spdlog::debug("found file {}", file);
    items.push_back(file);
  }
  spdlog::info("found {} backups, will be keep {} records", items.size(),
               count);
  if (items.size() <= count) {
    return;
  }
  std::sort(items.begin(), items.end(), std::greater<std::string>());
  items.erase(items.begin(), items.begin() + count);

  for (const auto& it : items) {
    spdlog::warn("delete file {}", it);
    std::filesystem::remove(it);
    const std::string md5 = std::format("{}.md5", it);
    if (std::filesystem::exists(md5)) {
      spdlog::warn("delete file {}", md5);
      std::filesystem::remove(md5);
    }
  }
}

static inline void render_template(const std::string& file,
                                   const std::string& tpl,
                                   const nlohmann::json& data) {
  spdlog::info("generate file {}", file);
  if (std::filesystem::exists(file)) {
    const std::string msg = std::format("file {} already exists", file);
    throw std::invalid_argument(msg);
  }
  std::ofstream out(file);
  inja::render_to(out, tpl, data);
}

void iris::Application::generate_timer(const std::string& name_) const {
  if (!iris::is_alphanumeric(name_)) {
    const std::string msg = std::format("incorrect timer name: {}", name_);
    throw std::invalid_argument(msg);
  }
  const std::string name = std::format("{}-{}", iris::PROJECT_NAME, name_);
  nlohmann::json data;
  data["name"] = name;

  render_template(std::format("{}.service", name), R"TPL(
[Unit]
Description=iris-{{ name }}.
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=root
Group=root
ExecStart=/usr/local/bin/iris dump -i input -o output -k 30 -z
WorkingDirectory=/var/lib/iris
Restart=always

[Install]
WantedBy=multi-user.target
)TPL",
                  data);

  render_template(std::format("{}.timer", name), R"TPL(
[Unit]
Description=iris-{{ name }}.

[Timer]
# OnBootSec=1hours
# OnUnitActiveSec=1day
OnCalendar=*-*-* 02:00:00

[Install]
WantedBy=timers.target
)TPL",
                  data);
}

void iris::Application::dump(const std::string& input,
                             const std::string& output_, bool compress,
                             size_t keep) const {
  if (keep < 1) {
    throw std::invalid_argument("keep count should be more than one");
  }
  const auto output = std::filesystem::absolute(output_);
  const auto package = iris::timestamp(input);
  const std::filesystem::path root = output / package;
  if (std::filesystem::exists(root)) {
    const std::string err =
        std::format("folder {} already exists", root.string());
    throw std::invalid_argument(err);
  }
  spdlog::info("backup {} into {} and keep recent {} files", input,
               root.string(), keep);
  {
    spdlog::debug("create folder {}", root.string());
    std::filesystem::create_directories(root);

    const std::string config_file = std::format("{}.toml", input);
    spdlog::info("load source configuration from {}", config_file);
    const toml::table config = toml::parse_file(config_file);
    std::optional<std::string_view> type =
        config["type"].value<std::string_view>();
    std::shared_ptr<iris::Storage> it;
    if (type == std::nullopt) {
      throw std::invalid_argument("empty type item");
    } else if (type.value() == "sync") {
      it = std::make_shared<iris::Filesystem>(config);
    } else if (type.value() == "dm8") {
      it = std::make_shared<iris::Dm8>(config);
    } else if (type.value() == "minio") {
      it = std::make_shared<iris::Minio>(config);
    } else {
      const std::string msg =
          std::format("unsupported storage {}", type.value());
      throw std::invalid_argument(msg);
    }
    it->dump(root);
  }

  {
    std::filesystem::current_path(output);
    spdlog::debug("changed working directory to {}",
                  std::filesystem::current_path().string());
  }

  const std::string tar = std::format("{}.tar", package);
  {
    spdlog::debug("compressing {}", tar);
    const auto res =
        iris::execute({"tar", "--remove-files", "-cf", tar, package});
    iris::check(res);
  }

  if (compress) {
    const std::string zip = std::format("{}.xz", tar);
    {
      spdlog::debug("compressing {}", zip);
      const auto res = iris::execute(
          {"xz", "-z", "-F", "xz", "-C", "sha256", "--best", tar});
      iris::check(res);
    }
    iris::md5(zip);
    keep_files(input + "-", ".tar.xz", keep);
  } else {
    iris::md5(tar);
    keep_files(input + "-", ".tar", keep);
  }

  const std::string output_config = std::format("{}.toml", output_);
  if (std::filesystem::exists(output_config)) {
    spdlog::info("load destination configuration from {}", output_config);
    const toml::table config = toml::parse_file(output_config);
    iris::Filesystem it(config);
    it.upload(output);
  }
}
int iris::Application::launch(int argc, char** argv) const {
  const std::string version =
      fmt::format("{}({})", iris::GIT_VERSION, iris::BUILD_TIME);
  argparse::ArgumentParser program(iris::PROJECT_NAME, version,
                                   argparse::default_arguments::help);

  program.add_description(iris::PROJECT_DESCRIPTION);
  program.add_epilog(iris::PROJECT_HOME);

  program.add_argument("-v", "--version").help("show version").flag();
  program.add_argument("-d", "--debug").help("run on debug mode").flag();

  argparse::ArgumentParser dump_command("dump");
  dump_command.add_description("Dump to file");
  dump_command.add_argument("-i", "--input")
      .help("input configuration file(toml)")
      .required();
  dump_command.add_argument("-o", "--output").help("output folder").required();
  dump_command.add_argument("-k", "--keep")
      .help("number of recent files to keep")
      .default_value(7)
      .scan<'i', int>()
      .required();
  dump_command.add_argument("-z", "--compress").help("compress").flag();
  program.add_subparser(dump_command);

  argparse::ArgumentParser restore_command("restore");
  restore_command.add_description("Restore from file");
  restore_command.add_argument("-i", "--input")
      .help("input file(xz)")
      .required();
  restore_command.add_argument("-o", "--output")
      .help("output configuration file(toml)")
      .required();
  program.add_subparser(restore_command);

  argparse::ArgumentParser generate_timer_command("generate-timer");
  generate_timer_command.add_description("Dump to file");
  generate_timer_command.add_argument("-n", "--name")
      .help("timer name(alphanumeric)")
      .required();
  program.add_subparser(generate_timer_command);

  try {
    program.parse_args(argc, argv);
  } catch (const std::exception& err) {
    spdlog::error("{}", err.what());
    return EXIT_FAILURE;
  }

  if (program.get<bool>("--version") == true) {
    std::cout << version << std::endl;
    return EXIT_SUCCESS;
  }

  {
    spdlog::set_level(program.get<bool>("--debug") == true
                          ? spdlog::level::debug
                          : spdlog::level::info);
    spdlog::debug("run on debug mode");
  }

  const std::string done = "done.";
  if (program.is_subcommand_used(dump_command)) {
    const std::string input = dump_command.get<std::string>("--input");
    const std::string output = dump_command.get<std::string>("--output");
    const int keep = dump_command.get<int>("--keep");
    const bool compress = dump_command.get<bool>("--compress");
    this->dump(input, output, compress, keep);
    spdlog::info(done);
    return EXIT_SUCCESS;
  }
  if (program.is_subcommand_used(restore_command)) {
    // TODO
    spdlog::info(done);
    return EXIT_SUCCESS;
  }
  if (program.is_subcommand_used(generate_timer_command)) {
    const std::string name = generate_timer_command.get<std::string>("--name");
    this->generate_timer(name);
    spdlog::info(done);
    return EXIT_SUCCESS;
  }

  std::cout << program.help().str() << std::endl;
  return EXIT_SUCCESS;
}
