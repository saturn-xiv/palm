#include "iris/application.hpp"
#include "iris/utils.hpp"
#include "iris/version.hpp"

#include <algorithm>
#include <cstdlib>

#include <spdlog/spdlog.h>

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
  data["version"] = iris::GIT_VERSION;

  render_template(std::format("{}.service", name), R"TPL(
[Unit]
Description=iris-{{ name }}({{ version }}).
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
Description=iris-{{ name }}({{ version }}).

[Timer]
# OnBootSec=1hours
# OnUnitActiveSec=1day
OnCalendar=*-*-* 02:00:00

[Install]
WantedBy=timers.target
)TPL",
                  data);

  spdlog::info(
      R"(please put them into /etc/systemd/system/ and then:
      sudo systemctl enable {}.timer
      sudo systemctl list-timers --all)",
      name);
}
