// #include "palm/theme.hpp"

// #include <exception>
// #include <fstream>
// #include <iostream>
// #include <locale>
// #include <streambuf>

// #include <boost/asio/connect.hpp>
// #include <boost/asio/ip/tcp.hpp>
// #include <boost/asio/ssl/error.hpp>
// #include <boost/asio/ssl/stream.hpp>
// #include <boost/beast/core.hpp>
// #include <boost/beast/http.hpp>
// #include <boost/beast/ssl.hpp>
// #include <boost/beast/version.hpp>
// #include <boost/log/trivial.hpp>

// // std::string palm::http::get(const std::string &host,
// //                             const std::string &target) {
// //   return get(host, 80, target);
// // }
// // std::string palm::http::get(const std::string &host, uint16_t port,
// //                             const std::string &target) {
// //   boost::asio::io_context ioc;
// //   boost::asio::ip::tcp::resolver resolver(ioc);
// //   boost::beast::tcp_stream stream(ioc);

// //   std::stringstream ps;
// //   ps << port;
// //   auto const results = resolver.resolve(host, ps.str());
// //   stream.connect(results);

// //   boost::beast::http::request<boost::beast::http::string_body> req{
// //       boost::beast::http::verb::get, target, 11};
// //   req.set(boost::beast::http::field::host, host);

// //   boost::beast::http::write(stream, req);
// //   boost::beast::flat_buffer buffer;
// //   boost::beast::http::response<boost::beast::http::string_body> res;

// //   boost::beast::http::read(stream, buffer, res);

// //   const auto status = res.result();
// //   const std::string body = res.body();
// //   const auto reason = res.reason();

// //   stream.socket().shutdown(boost::asio::ip::tcp::socket::shutdown_both);

// //   if (status == boost::beast::http::status::created ||
// //       status == boost::beast::http::status::ok) {
// //     return body;
// //   }
// //   throw std::invalid_argument(reason.data());
// // }

// // std::string palm::http::post(const std::string &host, const std::string
// // &target,
// //                              const std::string &form) {
// //   return post(host, 80, target, form);
// // }

// // std::string palm::http::post(const std::string &host, uint16_t port,
// //                              const std::string &target,
// //                              const std::string &form) {
// //   boost::asio::io_context ioc;
// //   boost::asio::ip::tcp::resolver resolver(ioc);
// //   boost::beast::tcp_stream stream(ioc);

// //   std::stringstream ps;
// //   ps << port;
// //   auto const results = resolver.resolve(host, ps.str());
// //   stream.connect(results);

// //   boost::beast::http::request<boost::beast::http::string_body> req{
// //       boost::beast::http::verb::post, target, 11};
// //   req.body() = form;
// //   req.set(boost::beast::http::field::host, host);
// //   req.set(boost::beast::http::field::content_type,
// //           "application/x-www-form-urlencoded;charset=UTF-8");
// //   //   req.set(boost::beast::http::field::content_length, form.length());

// //   boost::beast::http::write(stream, req);
// //   boost::beast::flat_buffer buffer;
// //   boost::beast::http::response<boost::beast::http::string_body> res;

// //   boost::beast::http::read(stream, buffer, res);

// //   const auto status = res.result();
// //   const std::string body = res.body();
// //   const auto reason = res.reason();

// //   stream.socket().shutdown(boost::asio::ip::tcp::socket::shutdown_both);

// //   if (status == boost::beast::http::status::created ||
// //       status == boost::beast::http::status::ok) {
// //     return body;
// //   }
// //   throw std::invalid_argument(reason.data());
// // }

// // std::string palm::https::get(const std::string &host,
// //                              const std::string &target) {
// //   return get(host, 443, target);
// // }
// // std::string palm::https::get(const std::string &host, uint16_t port,
// //                              const std::string &target) {
// //   boost::asio::io_context ioc;
// //   boost::asio::ssl::context ctx(boost::asio::ssl::context::tlsv12_client);
// //   ctx.set_verify_mode(boost::asio::ssl::verify_none);
// //   boost::asio::ip::tcp::resolver resolver(ioc);
// //   boost::beast::ssl_stream<boost::beast::tcp_stream> stream(ioc, ctx);

// //   std::stringstream ps;
// //   ps << port;
// //   auto const results = resolver.resolve(host, ps.str());
// //   boost::beast::get_lowest_layer(stream).connect(results);
// //   stream.handshake(boost::asio::ssl::stream_base::client);

// //   boost::beast::http::request<boost::beast::http::string_body> req{
// //       boost::beast::http::verb::get, target, 11};
// //   req.set(boost::beast::http::field::host, host);

// //   boost::beast::http::write(stream, req);
// //   boost::beast::flat_buffer buffer;
// //   boost::beast::http::response<boost::beast::http::string_body> res;
// //   boost::beast::http::read(stream, buffer, res);

// //   const auto status = res.result();
// //   const std::string body = res.body();
// //   const auto reason = res.reason();

// //   stream.shutdown();

// //   if (status == boost::beast::http::status::created ||
// //       status == boost::beast::http::status::ok) {
// //     return body;
// //   }
// //   throw std::invalid_argument(reason.data());
// // }
// // std::string palm::https::post(const std::string &host,
// //                               const std::string &target,
// //                               const std::string &form) {
// //   return post(host, 443, target, form);
// // }

// // std::string palm::https::post(const std::string &host, uint16_t port,
// //                               const std::string &target,
// //                               const std::string &form) {
// //   return post(host, 443, std::nullopt, target, form);
// // }

// // std::string palm::https::post(const std::string &host, const std::string
// // &user,
// //                               const std::string &password,
// //                               const std::string &target,
// //                               const std::string &form) {
// //   return post(host, 443, user, password, target, form);
// // }
// // //
// //
// https://www.ibm.com/docs/en/cics-ts/5.3?topic=concepts-http-basic-authentication
// // std::string palm::https::post(const std::string &host, uint16_t port,
// //                               const std::string &user,
// //                               const std::string &password,
// //                               const std::string &target,
// //                               const std::string &form) {
// //   std::stringstream ss;

// //   ss << "Basic " << user << ":" << password;
// //   return post(host, 443, ss.str(), target, form);
// // }
// // std::string palm::https::post(const std::string &host, uint16_t port,
// //                               std::optional<std::string> auth,
// //                               const std::string &target,
// //                               const std::string &form) {
// //   boost::asio::io_context ioc;
// //   boost::asio::ssl::context ctx(boost::asio::ssl::context::tlsv12_client);
// //   ctx.set_verify_mode(boost::asio::ssl::verify_none);
// //   boost::asio::ip::tcp::resolver resolver(ioc);
// //   boost::beast::ssl_stream<boost::beast::tcp_stream> stream(ioc, ctx);

// //   std::stringstream ps;
// //   ps << port;
// //   auto const results = resolver.resolve(host, ps.str());
// //   boost::beast::get_lowest_layer(stream).connect(results);
// //   stream.handshake(boost::asio::ssl::stream_base::client);

// //   boost::beast::http::request<boost::beast::http::string_body> req{
// //       boost::beast::http::verb::post, target, 11};
// //   req.body() = form;
// //   req.set(boost::beast::http::field::host, host);
// //   req.set(boost::beast::http::field::content_type,
// //           "application/x-www-form-urlencoded;charset=UTF-8");

// //   if (auth) {
// //     req.set(boost::beast::http::field::authorization, auth.value());
// //   }
// //   //   req.set(boost::beast::http::field::content_length, form.length());
// //   BOOST_LOG_TRIVIAL(debug) << req;

// //   boost::beast::http::write(stream, req);
// //   boost::beast::flat_buffer buffer;
// //   boost::beast::http::response<boost::beast::http::string_body> res;
// //   boost::beast::http::read(stream, buffer, res);
// //   BOOST_LOG_TRIVIAL(debug) << res;

// //   const auto status = res.result();
// //   const std::string body = res.body();
// //   const auto reason = res.reason();

// //   stream.shutdown();

// //   if (status == boost::beast::http::status::created ||
// //       status == boost::beast::http::status::ok) {
// //     return body;
// //   }
// //   throw std::invalid_argument(reason.data());
// // }
