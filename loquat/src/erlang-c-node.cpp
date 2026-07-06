#include <loquat/env.hpp>
#include <loquat/erlang.hpp>
#include <loquat/version.hpp>

#include <cstdio>

#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>

static int listen_on(uint16_t port) {
  int fd = socket(AF_INET, SOCK_STREAM, 0);
  if (fd < 0) {
    throw std::runtime_error("open socket");
  }

  int on = 1;
  setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &on, sizeof(on));

  struct sockaddr_in addr;

  memset((void*)&addr, 0, (size_t)sizeof(addr));
  addr.sin_family = AF_INET;
  addr.sin_port = htons(port);
  addr.sin_addr.s_addr = htonl(INADDR_ANY);

  if (bind(fd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
    throw std::runtime_error("binding socket");
  }

  if (listen(fd, 1 << 10) < 0) {
    throw std::runtime_error("listening socket");
  }
  return fd;
}

static int send_back(ei_cnode* ec, int fd, char* server_name) {
  spdlog::debug("sendback helloworld {}", server_name);
  ei_x_buff buf;
  ei_x_new_with_version(&buf);

  ei_x_encode_tuple_header(&buf, 2);
  ei_x_encode_pid(&buf, ei_self(ec));
  ei_x_encode_atom(&buf, "Hello world");

  spdlog::debug("### 1.1");
  if (ei_reg_send(ec, fd, server_name, buf.buff, buf.index) < 0) {
    throw std::runtime_error("send message failed");
  }
  spdlog::debug("### 1.2");
  ei_x_free(&buf);
  spdlog::debug("### 1.3");
}

static int get_process_name(ei_cnode* ec, int fd, erlang_pid* pid,
                            char* out_name, int max_len) {
  ei_x_buff args;
  ei_x_buff result;
  int index = 0;
  int version = 0;
  int tuple_size = 0;
  char atom_buf[MAXATOMLEN];

  // Initialize buffers
  ei_x_new(&args);
  ei_x_new(&result);

  // 1. Build the argument list: [Pid, registered_name]
  ei_x_encode_list_header(&args, 2);
  ei_x_encode_pid(&args, pid);
  ei_x_encode_atom(&args, "registered_name");
  ei_x_encode_empty_list(&args);

  // 2. Fire RPC call to erlang:process_info/2
  // args.buff + 1 bypasses the automatically added version magic number for the
  // RPC layer
  if (ei_rpc(ec, fd, "erlang", "process_info", args.buff + 1, args.index - 1,
             &result) < 0) {
    fprintf(stderr, "RPC system call failed.\n");
    ei_x_free(&args);
    ei_x_free(&result);
    return -1;
  }

  // 3. Decode the response buffer
  if (ei_decode_version(result.buff, &index, &version) < 0) {
    goto error;
  }

  // Check if process_info returned `[]` (unregistered process)
  int type;
  int size;
  ei_get_type(result.buff, &index, &type, &size);
  if (type == ERL_NIL_EXT) {
    printf("Process is alive but has no registered name.\n");
    ei_x_free(&args);
    ei_x_free(&result);
    return 0;
  }

  // If registered, it returns a 2-tuple: {registered_name, Name}
  if (ei_decode_tuple_header(result.buff, &index, &tuple_size) < 0 ||
      tuple_size != 2) {
    goto error;
  }

  // Decode the key atom: 'registered_name'
  if (ei_decode_atom(result.buff, &index, atom_buf) < 0) {
    goto error;
  }

  // Decode the value atom: The actual Process Name
  if (ei_decode_atom(result.buff, &index, out_name) < 0) {
    goto error;
  }

  // Clean up successfully
  ei_x_free(&args);
  ei_x_free(&result);
  return 1;

error:
  fprintf(stderr, "Failed to decode the process info response.\n");
  ei_x_free(&args);
  ei_x_free(&result);
  return -1;
}

loquat::erlang::CNode::CNode(const std::string& nodename,
                             const std::string& cookie, uint16_t port) {
  const std::string host = "10.0.0.198";
  spdlog::info("initializing erlang c-node on {}", host);

  uint32_t creation = static_cast<uint32_t>(time(NULL) + 1);
  struct in_addr addr;
  ei_cnode node;
  addr.s_addr = inet_addr(host.c_str());
  if (ei_connect_xinit(&node, "palm", "loquat", "loquat@palm.change-me.org",
                       &addr, "change-me", creation) < 0) {
    throw std::runtime_error("initializing erlang c-node");
  }

  spdlog::debug("try to listen on 0.0.0.0:{}", port);
  int sock = listen_on(port);

  spdlog::debug("publish the c-node socket");
  int pub = ei_publish(&node, port);

  spdlog::debug("accept connections");
  ErlConnect connection;
  int fd = ei_accept(&node, sock, &connection);
  if (fd == ERL_ERROR) {
    throw std::runtime_error("failed to accept connection");
  }

  erlang_msg msg;
  ei_x_buff buf;
  ei_x_new(&buf);

  for (;;) {
    int got = ei_xreceive_msg(fd, &msg, &buf);
    if (got == ERL_TICK) {
      continue;
    } else if (got == ERL_ERROR) {
      throw std::runtime_error("failed to receive message");
    } else {
      break;
    }
  }

  int index = 0;
  int version;
  ei_decode_version(buf.buff, &index, &version);
  spdlog::debug("index({}) version({})", index, version);
  /*
  int arity = 0;
  ei_decode_tuple_header(buf.buff, &index, &arity);
  if (arity != 2) {
    throw std::runtime_error("got wrong message");
  }
    */
  erlang_pid pid;
  ei_decode_pid(buf.buff, &index, &pid);
  spdlog::debug("index({}) pid({}@{})", index, pid.serial, pid.node);

  ei_x_free(&buf);

  {
    // std::string service;
    // service.resize(500, '\0');
    // if (get_process_name(&node, fd, &pid, service.data(), 500) < 0) {
    //   throw std::runtime_error("failed to get process name");
    // }

    std::string service = "my_demo_server";
    send_back(&node, fd, service.data());
    spdlog::debug("### 2.1");
  }

  spdlog::warn("shutdown the c-node server");
  close(fd);

  close(pub);
  close(sock);
}

loquat::erlang::CNode::~CNode() {}

void loquat::erlang::CNode::receive() const {
  // erlang_msg msg;
  // int index = 0;
  // int version;
  // int arity = 0;
  // erlang_pid pid;
  // ei_x_buff buf;
  // ei_x_new(&buf);
  // for (;;) {
  //   int got = ei_xreceive_msg(this->_sock_fd, &msg, &x);
  //   if (got == ERL_TICK) {
  //     continue;
  //   }
  //   if (got == ERL_ERROR) {
  //     spdlog::error("ei_xreceive_msg, got=={}", got);
  //     return;
  //   }
  //   break;
  // }
  // ei_decode_version(buf.buff, &index, &version);
  // ei_decode_tuple_header(buf.buff, &index, &arity);
  // if (arity != 2) {
  //   spdlog::error("got wrong message");
  //   return;
  // }
  // ei_decode_pid(buf.buff, &index, &pid);
}

void loquat::erlang::CNode::run() const {
  // TODO
}

std::vector<std::string> loquat::erlang::CNode::global_names() const {
  std::vector<std::string> items;
  // int count = 0;
  // int i = 0;

  // char** names = ei_global_names(&this->_node, this->_sock_fd, &count);

  // if (names != nullptr) {
  //   for (i = 0; i < count; i++) {
  //     std::string it = names[i];
  //     items.push_back(it);
  //   }

  //   free(names);
  // }

  return items;
}
