#include "palm/queue.hpp"
#include "palm/utils.hpp"

#include <boost/exception/diagnostic_information.hpp>

void palm::rabbitmq::Client::listen(const std::string &queue, bool exclusive,
                                    std::shared_ptr<QueueConsumer> consumer,
                                    std::chrono::seconds interval) {
  if (palm::is_stopped()) {
    return;
  }

  spdlog::info("listen on queue {} with consumer {}", queue, consumer->name());
  amqp_basic_consume(this->_connection, this->_channel,
                     amqp_cstring_bytes(queue.c_str()), amqp_empty_bytes, 0, 1,
                     exclusive ? 1 : 0, amqp_empty_table);
  this->check(amqp_get_rpc_reply(this->_connection), "consuming");

  for (;;) {
    amqp_maybe_release_buffers(this->_connection);

    amqp_envelope_t envelope;
    amqp_rpc_reply_t res =
        amqp_consume_message(this->_connection, &envelope, NULL, 0);

    if (AMQP_RESPONSE_NORMAL != res.reply_type) {
      break;
    }

    spdlog::debug(
        "delivery({}) exchange({} bytes, {}) routing key({} bytes, {})",
        envelope.delivery_tag, envelope.exchange.len,
        (char *)envelope.exchange.bytes, envelope.routing_key.len,
        (char *)envelope.routing_key.bytes);

    if ((envelope.message.properties._flags & AMQP_BASIC_CONTENT_TYPE_FLAG) &&
        (envelope.message.properties._flags & AMQP_BASIC_MESSAGE_ID_FLAG)) {
      std::string message_id(
          (char *)envelope.message.properties.message_id.bytes,
          envelope.message.properties.message_id.len);
      std::string content_type(
          (char *)envelope.message.properties.content_type.bytes,
          envelope.message.properties.content_type.len);
      std::vector<uint8_t> payload(
          (uint8_t *)envelope.message.body.bytes,
          (uint8_t *)envelope.message.body.bytes + envelope.message.body.len);
      spdlog::info("received ({}, {}) {} bytes", message_id, content_type,
                   payload.size());
      try {
        consumer->execute(message_id, content_type, payload);
      } catch (...) {
        spdlog::error(boost::current_exception_diagnostic_information());
      }
    }

    amqp_destroy_envelope(&envelope);

    if (palm::is_stopped()) {
      break;
    }
    if (interval.count() > 0) {
      std::this_thread::sleep_for(interval);
    }
  }
}

std::optional<std::string> palm::rabbitmq::Client::declare_queue(
    bool durable, bool exclusive, bool auto_delete) {
  spdlog::debug("declare an anonymous queue");
  const auto queue = amqp_queue_declare(
      this->_connection, this->_channel, amqp_empty_bytes, 0, durable ? 1 : 0,
      exclusive ? 1 : 0, auto_delete ? 1 : 0, amqp_empty_table);
  if (!this->check(amqp_get_rpc_reply(this->_connection),
                   "declaring an anonymous queue")) {
    return std::nullopt;
  }

  std::string name((char *)queue->queue.bytes, queue->queue.len);
  return name;
}
void palm::rabbitmq::Client::declare_queue(const std::string &name,
                                           bool durable, bool exclusive,
                                           bool auto_delete) {
  spdlog::debug("declare queue {}", name);
  amqp_queue_declare(this->_connection, this->_channel,
                     amqp_cstring_bytes(name.c_str()), 0, durable ? 1 : 0,
                     exclusive ? 1 : 0, auto_delete ? 1 : 0, amqp_empty_table);
  this->check(amqp_get_rpc_reply(this->_connection), "declaring a queue");
}
void palm::rabbitmq::Client::declare_exchange(const std::string &name,
                                              const std::string &type,
                                              bool durable, bool auto_delete) {
  spdlog::debug("declare exchange ({}, {})", name, type);
  amqp_exchange_declare(this->_connection, this->_channel,
                        amqp_cstring_bytes(name.c_str()),
                        amqp_cstring_bytes(type.c_str()), 0, durable ? 1 : 0,
                        auto_delete ? 1 : 0, 0, amqp_empty_table);
  this->check(amqp_get_rpc_reply(this->_connection), "declaring an exchange");
}

void palm::rabbitmq::Client::bind(const std::string &queue,
                                  const std::string &exchange,
                                  const std::string &binding_key) {
  spdlog::debug("bind {} to {} with {}", queue, exchange, binding_key);
  amqp_queue_bind(this->_connection, this->_channel,
                  amqp_cstring_bytes(queue.c_str()),
                  amqp_cstring_bytes(exchange.c_str()),
                  amqp_cstring_bytes(binding_key.c_str()), amqp_empty_table);
  this->check(amqp_get_rpc_reply(this->_connection), "bind");
}

void palm::rabbitmq::Client::send(const std::string &exchange,
                                  const std::string &routing_key,
                                  const std::string &content_type,
                                  const std::vector<uint8_t> payload) {
  const auto id = palm::uuid();
  spdlog::info("send message({}, {}, {} bytes) to ({}, {})", id, content_type,
               payload.size(), exchange, routing_key);
  amqp_basic_properties_t props;
  props._flags = AMQP_BASIC_CONTENT_TYPE_FLAG | AMQP_BASIC_DELIVERY_MODE_FLAG |
                 AMQP_BASIC_MESSAGE_ID_FLAG;
  props.message_id = amqp_cstring_bytes(id.c_str());
  props.content_type = amqp_cstring_bytes(content_type.c_str());
  props.delivery_mode = AMQP_DELIVERY_PERSISTENT;
  amqp_bytes_t body = {.len = payload.size(), .bytes = (void *)payload.data()};
  this->check(amqp_basic_publish(this->_connection, this->_channel,
                                 amqp_cstring_bytes(exchange.c_str()),
                                 amqp_cstring_bytes(routing_key.c_str()), 0, 0,
                                 &props, body),
              "basic publish");
}

palm::rabbitmq::Client::~Client() {
  this->check(
      amqp_channel_close(this->_connection, this->_channel, AMQP_REPLY_SUCCESS),
      "closing channel");
  this->check(amqp_connection_close(this->_connection, AMQP_REPLY_SUCCESS),
              "closing connection");
  this->check(amqp_destroy_connection(this->_connection), "ending connection");
}

bool palm::rabbitmq::Client::check(int status, const std::string &context) {
  if (status == AMQP_STATUS_OK) {
    return true;
  }
  spdlog::error("{}: {}", context, amqp_error_string2(status));
  return false;
}
bool palm::rabbitmq::Client::check(amqp_rpc_reply_t x,
                                   const std::string &context) {
  switch (x.reply_type) {
    case AMQP_RESPONSE_NORMAL:
      return true;

    case AMQP_RESPONSE_NONE:
      spdlog::error("{}: {}", context, " missing RPC reply type");
      break;

    case AMQP_RESPONSE_LIBRARY_EXCEPTION:
      spdlog::error("{}: {}", context, amqp_error_string2(x.library_error));
      break;

    case AMQP_RESPONSE_SERVER_EXCEPTION:
      switch (x.reply.id) {
        case AMQP_CONNECTION_CLOSE_METHOD: {
          amqp_connection_close_t *m =
              (amqp_connection_close_t *)x.reply.decoded;
          spdlog::error("{}: server connection error({}, {} bytes) {}", context,
                        m->reply_code, m->reply_text.len,
                        (char *)m->reply_text.bytes);
          break;
        }
        case AMQP_CHANNEL_CLOSE_METHOD: {
          amqp_channel_close_t *m = (amqp_channel_close_t *)x.reply.decoded;
          spdlog::error("{}: server channel error({}, {} bytes) {}", context,
                        m->reply_code, m->reply_text.len,
                        (char *)m->reply_text.bytes);
          break;
        }
        default:
          spdlog::error("{}: unknown server error method id {}", context,
                        x.reply.id);
          break;
      }
      break;
  }

  return false;
}

std::shared_ptr<palm::rabbitmq::Client> palm::rabbitmq::Config::open(
    int channel) const {
  std::shared_ptr<palm::rabbitmq::Client> it =
      std::make_shared<palm::rabbitmq::Client>();
  it->_connection = amqp_new_connection();
  it->_socket = amqp_tcp_socket_new(it->_connection);
  it->_channel = channel;
  if (it->_socket == nullptr) {
    spdlog::error("creating TCP socket");
    return nullptr;
  }

  spdlog::debug("open {}@{}:{}/{}", this->_user, this->_host, this->_port,
                this->_virtual_host);
  if (amqp_socket_open(it->_socket, this->_host.c_str(), this->_port) !=
      AMQP_STATUS_OK) {
    spdlog::error("opening TCP socket");
    return nullptr;
  }

  if (!it->check(amqp_login(it->_connection, this->_virtual_host.c_str(), 0,
                            (1 << 10) * 128, 0, AMQP_SASL_METHOD_PLAIN,
                            this->_user.c_str(), this->_password.c_str()),
                 "rabbitmq login")) {
    return nullptr;
  }

  {
    amqp_channel_open(it->_connection, it->_channel);
    if (!it->check(amqp_get_rpc_reply(it->_connection), "opening channel")) {
      return nullptr;
    }
  }
  return it;
}
