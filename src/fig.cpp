// #include "palm/fig.hpp"

// static void render_template(const std::filesystem::path& file,
//                             const std::string& tpl,
//                             const nlohmann::json& data) {
//   const auto fn = std::filesystem::temp_directory_path() / file;
//   BOOST_LOG_TRIVIAL(info) << "write to file " << fn.string();
//   if (std::filesystem::exists(fn)) {
//     throw std::invalid_argument("file already exists");
//   }
//   std::ofstream fd;
//   fd.open(fn);
//   inja::render_to(fd, tpl, data);
//   fd.close();
// }

// void palm::fig::Api::nginx_conf(const std::string& domain, bool ssl) {
//   const auto name = "www." + domain;
//   nlohmann::json data;
//   data["ssl"] = ssl;
//   data["domain"] = name;
//   data["port"] = this->port;
//   data["root"] = std::filesystem::current_path().string();

//   render_template((name + ".conf"), R"TPL(
// server {
//     {% if ssl %}
//     listen 443 ssl http2;
//     ssl_certificate /etc/nginx/{{ domain }}.crt;
//     ssl_certificate_key /etc/nginx/{{ domain }}.key;
//     {% else %}
//     listen 80 http2;
//     {% endif %}
//     # gzip config
//     gzip on;
//     gzip_min_length 1k;
//     gzip_comp_level 9;
//     gzip_types text/plain application/javascript application/x-javascript
//     text/css application/xml text/javascript application/x-httpd-php
//     image/jpeg image/gif image/png; gzip_vary on; gzip_disable "MSIE
//     [1-6]\.";

//     server_name {{ domain }};
//     root /var/www/{{ domain }};
//     access_log /var/log/nginx/ssl/{{ domain }}.access.log;
//     error_log  /var/log/nginx/ssl/{{ domain }}.error.log;

//     location /my/ {
//       alias /usr/share/palm/dashboard/;
//       try_files $uri $uri/ /my/index.html;
//     }

//     location /3rd/ {
//       alias /usr/share/palm/node_modules/;
//     }

//     location /assets/ {
//       alias /usr/share/palm/assets/;
//     }

//     location / {
//         proxy_set_header X-Forwarded-Proto http{% if ssl %}s{% endif %};
//         proxy_set_header X-Real-IP $remote_addr;
//         proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
//         proxy_set_header Host $http_host;
//         proxy_redirect off;
//         proxy_pass http://localhost:{{ port }};
//         proxy_set_header Upgrade $http_upgrade;
//         proxy_set_header Connection "upgrade";
//     }
// }
//   )TPL",
//                   data);
// }
// void palm::fig::Rpc::nginx_conf(const std::string& domain, bool ssl) {
//   const auto name = "rpc." + domain;
//   nlohmann::json data;
//   data["ssl"] = ssl;
//   data["domain"] = name;
//   data["port"] = this->port;

//   render_template((name + ".conf"), R"TPL(
// server {
//     {% if ssl %}
//     listen 443 ssl http2;
//     ssl_certificate /etc/nginx/ssl/{{ domain }}.crt;
//     ssl_certificate_key /etc/nginx/ssl/{{ domain }}.key;
//     {% else %}
//     listen 80 http2;
//     {% endif %}

//     server_name {{ domain }};
//     root /var/www/{{ domain }};
//     access_log /var/log/nginx/{{ domain }}.access.log;
//     error_log  /var/log/nginx/{{ domain }}.error.log;

//     location / {
//       grpc_pass grpc{% if ssl %}s{% endif %}://localhost:{{ port }};
//     }
// }
//   )TPL",
//                   data);
// }

// void palm::fig::Systemd::render(const std::string& name,
//                                 const std::string& action) {
//   nlohmann::json data;
//   data["domain"] = this->domain;
//   data["description"] = this->description;
//   data["action"] = action;

//   render_template((name + "." + this->domain + ".conf"), R"TPL(
// [Unit]
// Description={{description}}
// After=network.target

// [Service]
// Type=simple
// User=www-data
// Group=www-data
// WorkingDirectory=/var/www/{{ domain }}
// ExecStart=/usr/bin/palm {{ action }}
// Restart=on-failure

// [Install]
// WantedBy=multi-user.target
//     )TPL",
//                   data);
// }

// void palm::fig::Application::web() {
//   httplib::Server svr;
//   svr.set_logger([](const auto& req, const auto& res) {
//     BOOST_LOG_TRIVIAL(info)
//         << req.version << ' ' << req.method << ' ' << req.target;

//     for (auto it = req.headers.begin(); it != req.headers.end(); ++it) {
//       BOOST_LOG_TRIVIAL(debug) << it->first << ' ' << it->second;
//     }

//     BOOST_LOG_TRIVIAL(debug) << req.body;
//   });

//   // svr.set_error_handler([](const auto& req, auto& res) {
//   //   auto fmt = "<p>Error Status: <span style='color:red;'>%d</span></p>";
//   //   char buf[BUFSIZ];
//   //   snprintf(buf, sizeof(buf), fmt, res.status);
//   //   res.set_content(buf, "text/html");
//   // });

//   // svr.set_exception_handler([](const auto& req, auto& res, std::exception&
//   e)
//   // {
//   //   res.status = 500;
//   //   auto fmt = "<h1>Error 500</h1><p>%s</p>";
//   //   char buf[BUFSIZ];
//   //   snprintf(buf, sizeof(buf), fmt, e.what());
//   //   res.set_content(buf, "text/html");
//   // });

// #ifndef NDEBUG
//   {
//     const auto ret = svr.set_mount_point("/3rd", "./node_modules");
//     if (!ret) {
//       BOOST_LOG_TRIVIAL(error) << "can't mount 3rd";
//     }
//   }
//   {
//     const auto ret = svr.set_mount_point("/assets", "./assets");
//     if (!ret) {
//       BOOST_LOG_TRIVIAL(error) << "can't mount assets";
//     }
//   }
// #endif

//   svr.Get("/hi", [](const auto& req, auto& res) {
//     res.set_content("Hello World!", "text/plain");
//   });
//   BOOST_LOG_TRIVIAL(info) << "listen on http://0.0.0.0:"
//                           << this->config->api.port;
//   svr.listen("0.0.0.0", this->config->api.port);
// }
// void palm::fig::Application::rpc() {
//   //   TODO
// }
// void palm::fig::Application::worker() {
//   //   TODO
// }

// void palm::fig::Config::nginx_conf(const std::string& domain, bool ssl) {
//   this->api.nginx_conf(domain, ssl);
//   this->rpc.nginx_conf(domain, ssl);
// }
