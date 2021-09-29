// #pragma once

// #include "palm/pool.hpp"

// #include <amqp.h>
// #include <amqp_tcp_socket.h>
// #include <zmq.h>

// namespace palm {

// namespace rabbitmq {

// class Handler {
//  public:
//   virtual void execute(const std::string& id, const std::string&
//   content_type,
//                        const std::string& body) = 0;
// };

// class Connection {
//  public:
//   Connection(amqp_socket_t* socket, amqp_connection_state_t connection)
//       : socket(socket), connection(connection) {}
//   ~Connection();
//   void publish(const std::string& queue, const std::string& content_type,
//                const std::string& body);
//   void consume(const std::string& queue, std::shared_ptr<Handler> handler);

//  private:
//   void declare_queue(const std::string& name);

//   amqp_socket_t* socket;
//   amqp_connection_state_t connection;
// };

// class Config {
//  public:
//   Config(const std::string& host = "127.0.0.1", uint16_t port = 5672,
//          const std::string& user = "guest",
//          const std::string& password = "guest",
//          const std::string& virtual_host = "/", size_t size = 18)
//       : host(host),
//         port(port),
//         user(user),
//         password(password),
//         virtual_host(virtual_host),
//         size(size) {}

//   friend std::ostream& operator<<(std::ostream& out, const Config& self) {
//     out << "tcp://" << self.user << "@" << self.host << ":" << self.port <<
//     "/"
//         << self.virtual_host;
//     return out;
//   }

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["host"].value<std::string>();
//       if (it) {
//         this->host = it.value();
//       }
//     }
//     {
//       auto it = node["port"].value<uint16_t>();
//       if (it) {
//         this->port = it.value();
//       }
//     }
//     {
//       auto it = node["user"].value<std::string>();
//       if (it) {
//         this->user = it.value();
//       }
//     }
//     {
//       auto it = node["password"].value<std::string>();
//       if (it) {
//         this->password = it.value();
//       }
//     }
//     {
//       auto it = node["virtual-host"].value<std::string>();
//       if (it) {
//         this->virtual_host = it.value();
//       }
//     }
//     {
//       auto it = node["pool-size"].value<uint16_t>();
//       if (it) {
//         this->size = static_cast<size_t>(it.value());
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Config& self) {
//     node.insert_or_assign("host", self.host);
//     node.insert_or_assign("port", self.port);
//     node.insert_or_assign("user", self.user);
//     node.insert_or_assign("password", self.password);
//     node.insert_or_assign("virtual-host", self.virtual_host);
//     node.insert_or_assign("pool-size", static_cast<uint8_t>(self.size));

//     return node;
//   }

//   std::shared_ptr<Connection> build();

//   friend class palm::pool::Pool<Connection, Config>;

//  private:
//   std::string host;
//   std::string user;
//   std::string password;
//   uint16_t port;
//   std::string virtual_host;
//   size_t size;
// };

// using Pool = palm::pool::Pool<Connection, Config>;
// using PooledConnection = palm::pool::PooledConnection<Pool, Connection>;
// }  // namespace rabbitmq

// // https://github.com/booksbyus/zguide/tree/master/examples/C
// namespace zeromq {

// inline std::string tcp(const std::string& host, const uint16_t port) {
//   std::stringstream ss;
//   ss << "tcp://" << host << ":" << port;
//   return ss.str();
// }
// inline std::string tcp(const uint16_t port) { return tcp("*", port); }
// inline std::string ipc(const std::filesystem::path& file) {
//   auto it = file.parent_path();
//   if (!std::filesystem::exists(it)) {
//     std::filesystem::create_directory(it);
//   }
//   return "ipc://" + file.string();
// }

// class Subscriber {
//  public:
//   Subscriber(const std::string& address);
//   Subscriber(const std::string& address,
//              const std::vector<std::string>& envelopes);
//   ~Subscriber();
//   void start();

//  protected:
//   virtual void handle(const std::optional<std::string>& envelope,
//                       const std::string& payload) = 0;

//  private:
//   void open(const std::string& address,
//             const std::vector<std::string>& envelopes);
//   void* context;
//   void* socket;

//   bool envelope;
// };

// class Publisher {
//  public:
//   Publisher(const std::string& address);
//   ~Publisher();

//   void send(const std::string& envelope, const std::string& message);
//   void send(const std::string& message);
//   void send(const char* envelope, const char* message);
//   void send(const char* message);

//  private:
//   void* context;
//   void* socket;
// };

// class Producer {
//  public:
//   Producer(const std::string& address);
//   ~Producer();

//   void send(const std::string& message);
//   void send(const char* message);

//  private:
//   void* context;
//   void* socket;
// };

// class Consumer {
//  public:
//   Consumer(const std::string& address);
//   ~Consumer();
//   void start();

//  protected:
//   virtual void handle(const std::string& message) = 0;

//  private:
//   void* context;
//   void* socket;
// };

// }  // namespace zeromq
// }  // namespace palm
