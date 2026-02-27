#pragma once

#include "tulip/portal.hpp"

#include <boost/asio/dispatch.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/strand.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/config.hpp>

namespace tulip {
namespace http {
class Server {
 public:
  virtual void mount(
      tulip::portal::Context& context,
      const std::map<std::string, std::filesystem::path>& assets) = 0;
  virtual void startup(const std::string& host, uint16_t port) = 0;
  virtual void shutdown() = 0;
};
namespace cpp_httplib {
class Server : public tulip::http::Server {
 public:
  Server() : _server(std::make_shared<httplib::Server>()) {}
  void mount(
      tulip::portal::Context& context,
      const std::map<std::string, std::filesystem::path>& assets) override;
  void startup(const std::string& host, uint16_t port) override {
    this->_server->listen(host, port);
  }
  void shutdown() override { this->_server->stop(); }

 private:
  std::shared_ptr<httplib::Server> _server;
};
}  // namespace cpp_httplib
namespace boost_beast {

inline void fail(boost::beast::error_code ec, char const* what) {
  spdlog::error("{}: {}", what, ec.message());
}

class Session : public std::enable_shared_from_this<Session> {
 public:
  Session(boost::asio::ip::tcp::socket&& socket,
          tulip::portal::Context& context,
          const std::map<std::string, std::filesystem::path>& assets)
      : _stream(std::move(socket)), _context(context), _assets(assets) {}

  void run() {
    boost::asio::dispatch(_stream.get_executor(),
                          boost::beast::bind_front_handler(&Session::do_read,
                                                           shared_from_this()));
  }

  void do_read() {
    _request = {};
    _stream.expires_after(std::chrono::seconds(30));
    boost::beast::http::async_read(_stream, _buffer, _request,
                                   boost::beast::bind_front_handler(
                                       &Session::on_read, shared_from_this()));
  }

  void on_read(boost::beast::error_code ec, std::size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);

    if (ec == boost::beast::http::error::end_of_stream) {
      do_close();
      return;
    }

    if (ec) {
      fail(ec, "http read");
      return;
    }

    spdlog::info("HTTP({}) {} {}",
                 std::hash<std::thread::id>()(std::this_thread::get_id()),
                 this->_request.method_string(), this->_request.target());

    const auto start = std::chrono::high_resolution_clock::now();
    {
      auto res = this->handle(std::move(_request));
      bool keep_alive = res.keep_alive();
      boost::beast::async_write(
          this->_stream, std::move(res),
          boost::beast::bind_front_handler(&Session::on_write,
                                           shared_from_this(), keep_alive));
    }
    const auto stop = std::chrono::high_resolution_clock::now();

    spdlog::debug(
        "done({}) {}", std::hash<std::thread::id>()(std::this_thread::get_id()),
        std::format("{}", std::chrono::duration_cast<std::chrono::microseconds>(
                              stop - start)));
  }

  void on_write(bool keep_alive, boost::beast::error_code ec,
                std::size_t bytes_transferred) {
    boost::ignore_unused(bytes_transferred);

    if (ec) {
      fail(ec, "http on write");
      return;
    }

    if (!keep_alive) {
      do_close();
      return;
    }

    do_read();
  }

  void do_close() {
    boost::beast::error_code ec;
    _stream.socket().shutdown(boost::asio::ip::tcp::socket::shutdown_send, ec);
    if (ec) {
      fail(ec, "http close");
    }
  }

 private:
  boost::beast::http::message_generator handle(
      boost::beast::http::request<boost::beast::http::string_body>&& request);

  boost::beast::tcp_stream _stream;
  boost::beast::flat_buffer _buffer;
  boost::beast::http::request<boost::beast::http::string_body> _request;

  tulip::portal::Context _context;
  std::map<std::string, std::filesystem::path> _assets;
};

class Listener : public std::enable_shared_from_this<Listener> {
 public:
  Listener(boost::asio::io_context& ioc,
           boost::asio::ip::tcp::endpoint endpoint)
      : _io_context(ioc),
        _acceptor(boost::asio::make_strand(ioc))
  // ,
  // _context(context),
  // _assets(assets)
  // ,
  //    tulip::portal::Context context,
  //    std::map<std::string, std::filesystem::path> assets
  {
    boost::beast::error_code ec;

    this->_acceptor.open(endpoint.protocol(), ec);
    if (ec) {
      fail(ec, "http open");
      return;
    }

    this->_acceptor.set_option(boost::asio::socket_base::reuse_address(true),
                               ec);
    if (ec) {
      fail(ec, "http set option");
      return;
    }

    this->_acceptor.bind(endpoint, ec);
    if (ec) {
      fail(ec, "http bind");
      return;
    }

    this->_acceptor.listen(boost::asio::socket_base::max_listen_connections,
                           ec);
    if (ec) {
      fail(ec, "http listen");
      return;
    }
  }

  void run() { do_accept(); }

 private:
  void do_accept() {
    this->_acceptor.async_accept(boost::asio::make_strand(this->_io_context),
                                 boost::beast::bind_front_handler(
                                     &Listener::on_accept, shared_from_this()));
  }

  void on_accept(boost::beast::error_code ec,
                 boost::asio::ip::tcp::socket socket) {
    if (ec) {
      fail(ec, "http accept");
      return;
    } else {
      auto ss = std::make_shared<Session>(std::move(socket), this->_context,
                                          this->_assets);
      ss->run();
    }

    do_accept();
  }

  boost::asio::io_context& _io_context;
  boost::asio::ip::tcp::acceptor _acceptor;

  tulip::portal::Context _context;
  std::map<std::string, std::filesystem::path> _assets;
};

// https://beta.boost.org/doc/libs/1_85_0/libs/beast/example/http/server/async/http_server_async.cpp
class Server : public tulip::http::Server {
 public:
  Server(size_t threads)
      : _threads(threads), _pool(), _io_context(static_cast<int>(threads)) {
    this->_pool.reserve(threads - 1);
  }
  virtual void mount(
      tulip::portal::Context& context,
      const std::map<std::string, std::filesystem::path>& assets) override;
  virtual void startup(const std::string& host, uint16_t port) override;

  void shutdown() override;

 private:
  boost::asio::io_context _io_context;
  std::vector<std::thread> _pool;
  size_t _threads;
};
}  // namespace boost_beast
}  // namespace http
}  // namespace tulip
