// #include "palm/queue.hpp"

// #include <unistd.h>
// #include <sstream>

// #include <boost/log/trivial.hpp>
// #include <boost/uuid/uuid.hpp>
// #include <boost/uuid/uuid_generators.hpp>
// #include <boost/uuid/uuid_io.hpp>

// palm::rabbitmq::Connection::~Connection() {
//   BOOST_LOG_TRIVIAL(debug) << "close rabbitmq connection";
//   {
//     const auto reply =
//         amqp_channel_close(this->connection, 1, AMQP_REPLY_SUCCESS);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       BOOST_LOG_TRIVIAL(error)
//           << "failed on close rabbitmq channel: " << reply.reply_type;
//     }
//   }
//   {
//     const auto reply =
//         amqp_connection_close(this->connection, AMQP_REPLY_SUCCESS);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       BOOST_LOG_TRIVIAL(error)
//           << "failed on close rabbitmq connection: " << reply.reply_type;
//     }
//   }
//   {
//     const auto reply = amqp_destroy_connection(this->connection);
//     if (reply != AMQP_STATUS_OK) {
//       BOOST_LOG_TRIVIAL(error)
//           << "failed on ending rabbitmq connection: " << reply;
//     }
//   }
//   this->socket = nullptr;
// }
// void palm::rabbitmq::Connection::declare_queue(const std::string& name) {
//   amqp_queue_declare_ok_t* reply =
//       amqp_queue_declare(this->connection, 1,
//       amqp_cstring_bytes(name.c_str()),
//                          0, 1, 0, 0, amqp_empty_table);

//   {
//     const auto reply = amqp_get_rpc_reply(this->connection);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       throw std::invalid_argument("can't declaring a rabbitq queue");
//     }
//   }
// }
// void palm::rabbitmq::Connection::publish(const std::string& queue,
//                                          const std::string& content_type,
//                                          const std::string& body) {
//   this->declare_queue(queue);
//   BOOST_LOG_TRIVIAL(info) << "publish a " << queue << "://" << content_type
//                           << " message";

//   amqp_basic_properties_t prop;
//   prop.content_type = amqp_cstring_bytes(content_type.c_str());
//   prop._flags = AMQP_BASIC_CONTENT_TYPE_FLAG | AMQP_BASIC_DELIVERY_MODE_FLAG
//   |
//                 AMQP_BASIC_MESSAGE_ID_FLAG;
//   prop.delivery_mode = AMQP_DELIVERY_PERSISTENT;

//   boost::uuids::uuid uid = boost::uuids::random_generator()();
//   const auto id = boost::uuids::to_string(uid);
//   prop.message_id = amqp_cstring_bytes(id.c_str());

//   // RabbitMQ 3.x does not support the "immediate" flag according to
//   // https://www.rabbitmq.com/specification.html
//   const auto reply =
//       amqp_basic_publish(this->connection, 1, amqp_cstring_bytes(""),
//                          amqp_cstring_bytes(queue.c_str()), 1, 0, &prop,
//                          amqp_cstring_bytes(body.c_str()));
//   if (reply != AMQP_STATUS_OK) {
//     std::stringstream ss;
//     ss << "failed on public rabbitmq message(" << reply
//        << "): " << amqp_error_string2(reply);
//     throw std::runtime_error(ss.str());
//   }
//   BOOST_LOG_TRIVIAL(debug) << id;
// }
// // https://github.com/alanxz/rabbitmq-c/blob/master/tests/test_basic.c
// void palm::rabbitmq::Connection::consume(const std::string& queue,
//                                          std::shared_ptr<Handler> handler) {
//   this->declare_queue(queue);

//   {
//     // FIXME ack mode
//     amqp_basic_consume(this->connection, 1,
//     amqp_cstring_bytes(queue.c_str()),
//                        amqp_empty_bytes, 0, 1, 0, amqp_empty_table);
//     const auto reply = amqp_get_rpc_reply(this->connection);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       throw std::invalid_argument("can't consuming rabbitmq");
//     }
//   }

//   struct timeval timeout = {5, 0};

//   for (;;) {
//     amqp_envelope_t envelope;
//     const auto reply =
//         amqp_consume_message(this->connection, &envelope, &timeout, 0);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       if (!(reply.reply_type == AMQP_RESPONSE_LIBRARY_EXCEPTION &&
//             reply.library_error == AMQP_STATUS_TIMEOUT)) {
//         BOOST_LOG_TRIVIAL(error)
//             << "can't fetch a rabbitmq message " << reply.reply_type;
//       }
//       continue;
//     }

//     std::string id(
//         static_cast<char*>(envelope.message.properties.message_id.bytes),
//         envelope.message.properties.message_id.len);
//     std::string content_type(
//         static_cast<char*>(envelope.message.properties.content_type.bytes),
//         envelope.message.properties.content_type.len);
//     std::string body(static_cast<char*>(envelope.message.body.bytes),
//                      envelope.message.body.len);

//     try {
//       BOOST_LOG_TRIVIAL(info) << "receive a message " << id << "@" << queue;
//       BOOST_LOG_TRIVIAL(debug) << content_type << "\n" << body;
//       handler->execute(id, content_type, body);
//     } catch (const std::exception& ex) {
//       BOOST_LOG_TRIVIAL(error) << ex.what();
//     } catch (const std::string& ex) {
//       BOOST_LOG_TRIVIAL(error) << ex;
//     } catch (...) {
//       BOOST_LOG_TRIVIAL(error) << "unknown error";
//     }
//     amqp_destroy_envelope(&envelope);
//   }

//   // amqp_frame_t frame;
//   // for (;;) {
//   //   amqp_maybe_release_buffers(this->connection);
//   //   amqp_envelope_t envelope;
//   //   const auto it = amqp_consume_message(this->connection, &envelope,
//   NULL,
//   //   0);

//   //   if (AMQP_RESPONSE_NORMAL != it.reply_type) {
//   //     if (AMQP_RESPONSE_LIBRARY_EXCEPTION == it.reply_type &&
//   //         AMQP_STATUS_UNEXPECTED_STATE == it.library_error) {
//   //       if (AMQP_STATUS_OK !=
//   //           amqp_simple_wait_frame(this->connection, &frame)) {
//   //         return;
//   //       }

//   //       if (AMQP_FRAME_METHOD == frame.frame_type) {
//   //         switch (frame.payload.method.id) {
//   //           case AMQP_BASIC_RETURN_METHOD: {
//   //             amqp_message_t message;
//   //             const auto reply = amqp_read_message(this->connection,
//   //                                                  frame.channel,
//   &message,
//   //                                                  0);
//   //             if (AMQP_RESPONSE_NORMAL != reply.reply_type) {
//   //               return;
//   //             }

//   //             std::string id(
//   //                 static_cast<char*>(message.properties.message_id.bytes),
//   //                 message.properties.message_id.len);
//   //             std::string content_type(
//   // static_cast<char*>(message.properties.content_type.bytes),
//   //                 message.properties.content_type.len);
//   //             std::string body(static_cast<char*>(message.body.bytes),
//   //                              message.body.len);

//   //             try {
//   //               BOOST_LOG_TRIVIAL(info)
//   //                   << "receive a message " << id << "@" << queue;
//   //               BOOST_LOG_TRIVIAL(debug) << content_type << "\n" << body;
//   //               handler->execute(id, content_type, body);
//   //             } catch (const std::exception& ex) {
//   //               BOOST_LOG_TRIVIAL(error) << ex.what();
//   //             } catch (const std::string& ex) {
//   //               BOOST_LOG_TRIVIAL(error) << ex;
//   //             } catch (...) {
//   //               BOOST_LOG_TRIVIAL(error) << "unknown error";
//   //             }
//   //             amqp_destroy_message(&message);
//   //           } break;
//   //           case AMQP_BASIC_ACK_METHOD:
//   //             break;
//   //           case AMQP_CHANNEL_CLOSE_METHOD:
//   //             return;
//   //           case AMQP_CONNECTION_CLOSE_METHOD:
//   //             return;
//   //           default:
//   //             BOOST_LOG_TRIVIAL(warning) << "an unexpected method was
//   //             received "
//   //                                        << frame.payload.method.id;
//   //             return;
//   //         }
//   //       }
//   //     }
//   //   } else {
//   //     amqp_destroy_envelope(&envelope);
//   //   }
//   // }
// }

// std::shared_ptr<palm::rabbitmq::Connection> palm::rabbitmq::Config::build() {
//   amqp_connection_state_t con = amqp_new_connection();
//   amqp_socket_t* sck = amqp_tcp_socket_new(con);
//   if (sck == nullptr) {
//     throw std::invalid_argument("can't create rabbitmq socket");
//   }

//   {
//     const auto status =
//         amqp_socket_open(sck, this->host.c_str(),
//         static_cast<int>(this->port));
//     if (status != 0) {
//       throw std::invalid_argument("can't create rabbitmq host");
//     }
//   }

//   {
//     // std::numeric_limits<unsigned short>::max()
//     const auto reply = amqp_login(con, this->virtual_host.c_str(), 0, 131072,
//     0,
//                                   AMQP_SASL_METHOD_PLAIN, this->user.c_str(),
//                                   this->password.c_str());
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       throw std::invalid_argument("can't login rabbitmq");
//     }
//   }
//   {
//     amqp_channel_open(con, 1);
//     const auto reply = amqp_get_rpc_reply(con);
//     if (reply.reply_type != AMQP_RESPONSE_NORMAL) {
//       throw std::invalid_argument("can't open rabbitmq's channel");
//     }
//   }
//   auto it = std::make_shared<Connection>(sck, con);
//   return it;
// }

// // https://github.com/booksbyus/zguide/blob/master/examples/C/zhelpers.h
// void send_message(void* socket, const char* buffer, int flags = 0) {
//   zmq_msg_t message;
//   zmq_msg_init_size(&message, strlen(buffer));
//   memcpy(zmq_msg_data(&message), buffer, strlen(buffer));
//   if (zmq_msg_send(&message, socket, flags) == -1) {
//     BOOST_LOG_TRIVIAL(error) << "failed on send zmq message ";
//   }
//   zmq_msg_close(&message);
// }

// std::string receive_message(void* socket) {
//   zmq_msg_t reply;
//   zmq_msg_init(&reply);
//   if (zmq_msg_recv(&reply, socket, 0) < 0) {
//     BOOST_LOG_TRIVIAL(error) << "failed to receive a zmq message";
//   }
//   auto size = zmq_msg_size(&reply);
//   std::string line((char*)zmq_msg_data(&reply), size);
//   BOOST_LOG_TRIVIAL(info) << "get line: " << line;
//   // zmq_msg_more(&reply)
//   zmq_msg_close(&reply);
//   return line;
// }

// palm::zeromq::Publisher::Publisher(const std::string& address) {
//   BOOST_LOG_TRIVIAL(info) << "start publisher at " << address;
//   this->context = zmq_ctx_new();
//   this->socket = zmq_socket(context, ZMQ_PUB);
//   zmq_bind(socket, address.c_str());
// }

// palm::zeromq::Publisher::~Publisher() {
//   BOOST_LOG_TRIVIAL(info) << "close publisher";
//   zmq_close(this->socket);
//   zmq_ctx_destroy(this->context);
//   this->socket = nullptr;
// }

// void palm::zeromq::Publisher::send(const std::string& envelope,
//                                    const std::string& message) {
//   this->send(envelope.c_str(), message.c_str());
// }

// void palm::zeromq::Publisher::send(const std::string& message) {
//   this->send(message.c_str());
// }

// void palm::zeromq::Publisher::send(const char* envelope, const char* message)
// {
//   BOOST_LOG_TRIVIAL(info) << "pub: " << message << "@" << envelope;
//   send_message(this->socket, envelope, ZMQ_SNDMORE);
//   send_message(this->socket, message);
// }

// void palm::zeromq::Publisher::send(const char* message) {
//   BOOST_LOG_TRIVIAL(info) << "pub: " << message;
//   send_message(this->socket, message);
// }

// palm::zeromq::Subscriber::Subscriber(const std::string& address) {
//   std::vector<std::string> envelopes;
//   this->open(address, envelopes);
// }

// palm::zeromq::Subscriber::Subscriber(
//     const std::string& address, const std::vector<std::string>& envelopes) {
//   this->open(address, envelopes);
// }

// void palm::zeromq::Subscriber::open(const std::string& address,
//                                     const std::vector<std::string>&
//                                     envelopes) {
//   BOOST_LOG_TRIVIAL(info) << "start subscriber on " << address;
//   this->envelope = !envelopes.empty();
//   this->context = zmq_ctx_new();
//   this->socket = zmq_socket(this->context, ZMQ_SUB);
//   zmq_connect(this->socket, address.c_str());
//   for (const auto& it : envelopes) {
//     BOOST_LOG_TRIVIAL(info) << "subscribe on " << it;
//     zmq_setsockopt(this->socket, ZMQ_SUBSCRIBE, it.c_str(), 1);
//   }
// }

// palm::zeromq::Subscriber::~Subscriber() {
//   zmq_close(this->socket);
//   zmq_ctx_destroy(this->context);
// }

// void palm::zeromq::Subscriber::start() {
//   while (1) {
//     if (this->envelope) {
//       const auto evp = receive_message(this->socket);
//       const auto msg = receive_message(this->socket);
//       BOOST_LOG_TRIVIAL(info) << "sub: " << msg << "@" << evp;
//       this->handle(evp, msg);
//     } else {
//       const auto msg = receive_message(this->socket);
//       BOOST_LOG_TRIVIAL(info) << "sub: " << msg;
//       this->handle(std::nullopt, msg);
//     }
//   }
// }

// palm::zeromq::Producer::Producer(const std::string& address) {
//   BOOST_LOG_TRIVIAL(info) << "start producer on " << address;
//   this->context = zmq_ctx_new();
//   this->socket = zmq_socket(this->context, ZMQ_PUSH);
//   zmq_connect(this->socket, address.c_str());
// }

// palm::zeromq::Producer::~Producer() {
//   zmq_close(this->socket);
//   zmq_ctx_destroy(this->context);
// }

// void palm::zeromq::Producer::send(const std::string& message) {
//   this->send(message.c_str());
// }

// void palm::zeromq::Producer::send(const char* message) {
//   BOOST_LOG_TRIVIAL(info) << "push: " << message;
//   send_message(this->socket, message, 0);
// }

// palm::zeromq::Consumer::Consumer(const std::string& address) {
//   BOOST_LOG_TRIVIAL(info) << "start consumer at " << address;
//   this->context = zmq_ctx_new();
//   this->socket = zmq_socket(context, ZMQ_PULL);
//   zmq_bind(socket, address.c_str());
// }

// palm::zeromq::Consumer::~Consumer() {
//   zmq_close(this->socket);
//   zmq_ctx_destroy(this->context);
// }

// void palm::zeromq::Consumer::start() {
//   while (true) {
//     const auto msg = receive_message(this->socket);
//     BOOST_LOG_TRIVIAL(info) << "pull: " << msg;
//     this->handle(msg);
//   }
// }
