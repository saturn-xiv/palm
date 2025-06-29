#pragma once

#include <chrono>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include <rabbitmq-c/amqp.h>
#include <rabbitmq-c/tcp_socket.h>
#include <toml++/toml.hpp>

namespace basil {

class QueueConsumer {
 public:
  virtual std::string name() = 0;
  virtual void execute(const std::string& id, const std::string& content_type,
                       const std::vector<uint8_t> payload) = 0;
};

namespace rabbitmq {

class Client {
 public:
  ~Client();
  friend class Config;

  void ping();

  inline void produce(const std::string& queue, const std::string& content_type,
                      const std::vector<uint8_t> payload) {
    this->declare_queue(queue);
    this->send("", queue, content_type, payload);
  }
  inline void consume(const std::string& queue,
                      std::shared_ptr<QueueConsumer> consumer,
                      std::chrono::seconds interval = std::chrono::seconds(0)) {
    this->declare_queue(queue);
    this->listen(queue, false, consumer, interval);
  }
  inline void publish(const std::string& exchange,
                      const std::string& content_type,
                      const std::vector<uint8_t> payload) {
    this->declare_exchange(exchange, "fanout");
    this->send(exchange, "", content_type, payload);
  }
  inline void subscribe(
      const std::string& exchange, std::shared_ptr<QueueConsumer> consumer,
      std::chrono::seconds interval = std::chrono::seconds(0)) {
    this->declare_exchange(exchange, "fanout");
    const auto queue = this->declare_queue();
    this->bind(queue.value(), exchange, "");
    this->listen(queue.value(), true, consumer, interval);
  }

 private:
  bool check(int status, const std::string& context);
  bool check(amqp_rpc_reply_t reply, const std::string& context);
  void send(const std::string& exchange, const std::string& routing_key,
            const std::string& content_type,
            const std::vector<uint8_t> payload);
  void declare_queue(const std::string& name, bool durable = true,
                     bool exclusive = false, bool auto_delete = false);
  std::optional<std::string> declare_queue(bool durable = false,
                                           bool exclusive = true,
                                           bool auto_delete = true);
  void declare_exchange(const std::string& name, const std::string& type,
                        bool durable = true, bool auto_delete = false);
  void bind(const std::string& queue, const std::string& exchange,
            const std::string& binding_key);
  void listen(const std::string& queue, bool exclusive,
              std::shared_ptr<QueueConsumer> consumer,
              std::chrono::seconds interval);

  amqp_connection_state_t _connection;
  amqp_socket_t* _socket;
  int _channel;
};

class Config {
 public:
  Config(const std::string& host = "127.0.0.1", uint16_t port = 5672,
         const std::string& user = "guest",
         const std::string& password = "password",
         const std::string& virtual_host = "/")
      : _host(host),
        _port(port),
        _user(user),
        _password(password),
        _virtual_host(virtual_host) {}
  std::shared_ptr<Client> open(int channel = 1) const;
  inline void set_virtual_host(const std::string& virtual_host) {
    this->_virtual_host = virtual_host;
  }
  inline void set_user(const std::string& user) { this->_user = user; }
  inline void set_password(const std::string& password) {
    this->_password = password;
  }

 private:
  std::string _host;
  uint16_t _port;
  std::string _user;
  std::string _password;
  std::string _virtual_host;
};
}  // namespace rabbitmq

}  // namespace basil
