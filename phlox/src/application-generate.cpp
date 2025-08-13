#include "phlox/application.hpp"
#include "phlox/services.hpp"

void phlox::Application::generate_token(const toml::table& config,
                                        const std::string& username,
                                        uint8_t years) {
  spdlog::info("generate token for user {} with {} years", username, years);
  auto jwt = this->jwt(config);
  const auto token = jwt->sign(phlox::CurrentUser::ISSUER, username,
                               {phlox::CurrentUser::WEB_AUDIENCE}, std::nullopt,
                               std::chrono::years{years});
  std::cout << token << std::endl;
  spdlog::info("done.");
}

void phlox::Application::generate_etc(const std::string& domain) {
  const auto etc = std::filesystem::path("etc") / domain;
  if (std::filesystem::exists(etc)) {
    spdlog::warn("folder {} exists", etc.string());
    return;
  }

  spdlog::debug("generate folder {}", etc.string());
  std::filesystem::create_directories(etc);

  {
    auto file = etc / "nginx.conf";
    spdlog::info("generate file {}", file.string());
    nlohmann::json data = {
        {"domain", domain},
        {
            "api",
            {{"hosts", {"192.168.21", "192.168.22", "192.168.23"}},
             {"port", 8080}},
        },
    };

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"NGINX(
# -----------------------------------------------------------------------------

# https://nginx.org/en/docs/http/ngx_http_upstream_module.html
upstream api_{{ domain }} {
## for host in api.hosts
  server {{ host }}:{{ api.port }};
## endfor
}

# https://pro.ant.design/docs/deploy/#use-nginx
server {
  listen 80;

  server_name {{ domain }};
  access_log /var/log/nginx/{{ domain }}.access.log;
  error_log  /var/log/nginx/{{ domain }}.error.log;

  gzip on;
  gzip_comp_level 9;
  gzip_min_length 1k;
  gzip_types text/plain text/css application/xml application/javascript;
  gzip_vary on;
  client_max_body_size 128M;

  location /my/ {
    alias /usr/share/palm/phlox/dashboard/;
    try_files $uri $uri/ /my/index.html;

    location ~* \.(css|js|png|jpg|jpeg|gif|gz|svg|mp4|ogg|ogv|webm|htc|xml|woff)$ {
      access_log off;
      expires max;
    }
  }
  
  location / {
    proxy_set_header X-Forwarded-Proto http;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_redirect off;
    proxy_pass http://api_{{ domain }};
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }
}

# -----------------------------------------------------------------------------
)NGINX";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  {
    const auto file = etc / std::format("api.{}.conf", domain);
    spdlog::info("generate file {}", file.string());

    nlohmann::json data = {{"domain", domain}, {"port", 8080}};

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"SYSTEMD(
[Unit]
Description=HTTP api service for {{ domain }}
After=rpc.{{ domain }}.service

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/phlox -c /etc/palm/{{ domain }}-http.toml -p {{ port }}
# or always, on-abort, on-failure, etc
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
)SYSTEMD";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  {
    const auto file = etc / std::format("rpc.{}.conf", domain);
    spdlog::info("generate file {}", file.string());

    nlohmann::json data = {{"domain", domain}, {"port", 9090}};

    spdlog::debug("args:\n{}", data.dump(4));
    const std::string tpl = R"SYSTEMD(
[Unit]
Description=gRPC service for {{ domain }}
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data
WorkingDirectory=/var/lib/{{ domain }}
ExecStart=/usr/bin/phlox -c /etc/{{ domain }}-rpc.toml rpc -p {{ port }}
# or always, on-abort, on-failure, etc
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
)SYSTEMD";
    std::ofstream out(file);
    inja::render_to(out, tpl, data);
  }

  spdlog::info("done.");
}
