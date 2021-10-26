#include "palm/fig.hpp"
#include "palm/cache.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"

void palm::fig::app::start_web() {
  // TODO
}
void palm::fig::app::start_rpc() {
  // TODO
}
void palm::fig::app::start_worker() {
  // TODO
}
void palm::fig::app::db_migrate() {
  // TODO
}
void palm::fig::app::db_rollback() {
  // TODO
}
void palm::fig::app::db_status() {
  // TODO
}
void palm::fig::app::cache_list() {
  // TODO
}
void palm::fig::app::cache_clear() {
  // TODO
}
void palm::fig::app::generate_config(const std::filesystem::path& file) {
  BOOST_LOG_TRIVIAL(info) << "generate config file " << file;
  if (std::filesystem::exists(file)) {
    std::stringstream ss;
    ss << file << " already exists";
    throw std::invalid_argument(ss.str());
  }
  toml::table root;
  {
    palm::redis::Config cfg;
    toml::table it;
    it << cfg;
    root.insert_or_assign("redis", it);
  }
  {
    palm::postgresql::Config cfg;
    toml::table it;
    it << cfg;
    root.insert_or_assign("postgresql", it);
  }
  {
    palm::rabbitmq::Config cfg;
    toml::table it;
    it << cfg;
    root.insert_or_assign("rabbitmq", it);
  }
  std::ofstream out(file, std::ios::out);
  out << root;
  out.close();
}
void palm::fig::app::generate_systemd(const std::string& domain) {
  // TODO
}
void palm::fig::app::generate_db_migration(const std::string& name) {
  // TODO
}
void palm::fig::app::generate_nginx(const std::string& domain, const bool ssl) {
  const std::string web = R"(
# https://pro.ant.design/docs/deploy/
server {
{% if ssl %}
  listen 443 ssl http2 default_server;
  ssl_certificate /etc/nginx/ssl/www.{{ domain }}.crt;
  ssl_certificate_key /etc/nginx/ssl/www.{{ domain }}.key;
{% else %}
  listen 80 http2 default_server;
{% endif %}

  server_name www.{{ domain }};
  root /var/www/{{ domain }}/htdocs;
  access_log /var/log/nginx/www.{{ domain }}.access.log;
  error_log  /var/log/nginx/www.{{ domain }}.error.log;

  location /my/ {
    alias /usr/share/{{ name }}/dashboard/;
    try_files $uri $uri/ /my/index.html;
  }
  location /3rd/ {
    alias /usr/share/{{ name }}/node_modules/;    
  }
  location /assets/ {
    alias /usr/share/{{ name }}/assets/;    
  }
    
  location / {
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;    
    proxy_set_header Host $http_host;
    proxy_redirect off;
    proxy_pass http://127.0.0.1:{{ port }};
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
  }
}

{% if ssl %}
server {
  if ($host = www.{{ domain }}) {
    return 301 https://$host$request_uri;
  }
  listen 80;
  server_name www.{{ domain }};
  return 404;
}
{% endif %}
)";
  const std::string rpc = R"(
# https://www.nginx.com/blog/nginx-1-13-10-grpc/
server {
{% if ssl %}
  listen 443 ssl http2 default_server;
  ssl_certificate /etc/nginx/ssl/rpc.{{ domain }}.crt;
  ssl_certificate_key /etc/nginx/ssl/rpc.{{ domain }}.key;
{% else %}
  listen 80 http2 default_server;
{% endif %}
  server_name rpc.{{ domain }};
  access_log /var/log/nginx/rpc.{{ domain }}.access.log;
  error_log  /var/log/nginx/rpc.{{ domain }}.error.log;

  location / {
    grpc_pass grpc://127.0.0.1:{{ port }};
  }
}
)";
  const std::string s3 = R"(
# https://docs.min.io/docs/setup-nginx-proxy-with-minio.html
server {
  {% if ssl %}
  listen 443 ssl default_server;
  ssl_certificate /etc/nginx/ssl/s3.{{ domain }}.crt;
  ssl_certificate_key /etc/nginx/ssl/s3.{{ domain }}.key;
{% else %}
  listen 80 default_server;
{% endif %}
  server_name s3.{{ domain }};

  ignore_invalid_headers off;
  client_max_body_size 0;
  proxy_buffering off;

  location / {
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header Host $http_host;

    proxy_connect_timeout 300;
  
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    chunked_transfer_encoding off;

    proxy_pass http://127.0.0.1:9000;
  }
}
)";
  {
    const nlohmann::json data = {
        {"domain", domain}, {"ssl", ssl}, {"name", "palm"}};
    inja::render(web, data);
  }
}
