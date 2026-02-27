#include "tulip/accounting.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"
#include "tulip/http.hpp"

// static std::variant<
//     std::unique_ptr<
//         boost::beast::http::response<boost::beast::http::file_body>>,
//     std::unique_ptr<
//         boost::beast::http::response<boost::beast::http::string_body>>>
// handle_request(
//     tulip::portal::Context& ctx,
//     const boost::beast::http::request<boost::beast::http::string_body> req,
//     const std::map<std::string, std::filesystem::path>& assets) {
//   if (req.target().empty() || req.target()[0] != '/' ||
//       req.target().find("..") != boost::beast::string_view::npos) {
//     return palm::http::bad_request(req);
//   }
//   if (req.method() == boost::beast::http::verb::get ||
//       req.method() == boost::beast::http::verb::head) {
//   }
//   if (req.method() == boost::beast::http::verb::post) {
//   }
//     return palm::http::not_found(req);
// }

boost::beast::http::message_generator tulip::http::boost_beast::Session::handle(
    boost::beast::http::request<boost::beast::http::string_body>&& req) {
  spdlog::info("{} {}", req.method_string(), req.target());
  //   TODO

  return palm::http::not_found(req);
}

void tulip::http::boost_beast::Server::mount(
    tulip::portal::Context& context,
    const std::map<std::string, std::filesystem::path>& assets) {
  // TODO
}

void tulip::http::boost_beast::Server::startup(const std::string& host,
                                               uint16_t port) {
  const auto endpoint =
      boost::asio::ip::tcp::endpoint{boost::asio::ip::make_address(host), port};

  auto server = std::make_shared<Listener>(this->_io_context, endpoint);
  server->run();

  std::vector<std::thread> workers;
  workers.reserve(this->_threads);
  for (auto i = this->_threads; i > 0; --i) {
    workers.emplace_back([this] { this->_io_context.run(); });
  }
  for (auto& it : workers) {
    it.join();
  }
}

void tulip::http::boost_beast::Server::shutdown() { this->_io_context.stop(); }
