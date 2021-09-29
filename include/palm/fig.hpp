// #pragma once

// #include "palm/cache.hpp"
// #include "palm/crypto.hpp"
// #include "palm/orm.hpp"
// #include "palm/queue.hpp"
// #include "palm/theme.hpp"
// #include "palm/twilio.hpp"

// namespace palm {
// namespace fig {

// class Api {
//  public:
//   Api(uint16_t port = 8080) : port(port) {}
//   friend std::ostream& operator<<(std::ostream& out, const Api& self) {
//     out << "http://0.0.0.0:" << self.port;
//     return out;
//   }

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["port"].value<uint16_t>();
//       if (it) {
//         this->port = it.value();
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Api& self) {
//     node.insert_or_assign("port", self.port);

//     return node;
//   }

//   // https://pro.ant.design/docs/deploy/
//   void nginx_conf(const std::string& domain, bool ssl = false);

//   uint16_t port;
// };

// class Rpc {
//  public:
//   Rpc(uint16_t port = 8088) : port(port) {}
//   // https://www.nginx.com/blog/nginx-1-13-10-grpc/
//   void nginx_conf(const std::string& domain, bool ssl = false);
//   friend std::ostream& operator<<(std::ostream& out, const Rpc& self) {
//     out << "http://0.0.0.0:" << self.port;
//     return out;
//   }

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["port"].value<uint16_t>();
//       if (it) {
//         this->port = it.value();
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Rpc& self) {
//     node.insert_or_assign("port", self.port);
//     return node;
//   }

//   uint16_t port;
// };

// class Config {
//  public:
//   Config(const std::string& secrets =
//              palm::base64::encode(palm::random::bytes(32)))
//       : secrets(secrets) {}
//   void operator=(const toml::table& node) {
//     {
//       const auto secrets = node["secrets"].value<std::string>();
//       if (secrets) {
//         this->secrets = secrets.value();
//       }
//     }

//     this->redis = *node["redis"].as_table();
//     this->rabbitmq = *node["rabbitmq"].as_table();
//     this->postgresql = *node["postgresql"].as_table();
//     this->twilio = *node["twilio"].as_table();
//     this->api = *node["api"].as_table();
//     this->rpc = *node["rpc"].as_table();
//   }
//   friend toml::table& operator<<(toml::table& node, const Config& self) {
//     node.insert_or_assign("secrets", self.secrets);
//     {
//       toml::table it;
//       it << self.api;
//       node.insert_or_assign("api", it);
//     }
//     {
//       toml::table it;
//       it << self.rpc;
//       node.insert_or_assign("rpc", it);
//     }
//     {
//       toml::table it;
//       it << self.postgresql;
//       node.insert_or_assign("postgresql", it);
//     }
//     {
//       toml::table it;
//       it << self.redis;
//       node.insert_or_assign("redis", it);
//     }
//     {
//       toml::table it;
//       it << self.rabbitmq;
//       node.insert_or_assign("rabbitmq", it);
//     }
//     {
//       toml::table it;
//       it << self.twilio;
//       node.insert_or_assign("twilio", it);
//     }
//     return node;
//   }

//   void nginx_conf(const std::string& domain, bool ssl);

//   std::string secrets;
//   palm::postgresql::Config postgresql;
//   palm::redis::Config redis;
//   palm::rabbitmq::Config rabbitmq;
//   palm::twilio::Client twilio;
//   Api api;
//   Rpc rpc;
// };

// class Systemd {
//  public:
//   Systemd(const std::string& domain, const std::string& description)
//       : domain(domain), description(description) {}
//   inline void web() { this->render("www", "web"); }
//   inline void rpc() { this->render("rpc", "rpc"); }
//   inline void worker() { this->render("job", "worker"); }

//  private:
//   std::string domain;
//   std::string description;

//   void render(const std::string& name, const std::string& action);
// };

// class Application {
//  public:
//   Application(std::shared_ptr<Config> config) : config(config) {}
//   void web();
//   void rpc();
//   void worker();

//  private:
//   std::shared_ptr<Config> config;
// };

// }  // namespace fig
// }  // namespace palm
