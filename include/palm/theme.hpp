// #pragma once

// #include "palm/env.hpp"

// #include <boost/beast/http.hpp>

// namespace palm {

// // namespace http {

// // struct HttpResponse {
// //   std::string content_type;
// //   boost::beast::http::status status;
// //   std::string body;
// //   std::string reason;
// // };

// // struct HttpRequest {
// //   std::string path;
// //   std::string content_type;
// //   std::optional<std::string> body;
// // };

// // //
// //
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
// // class Client {
// //  public:
// //   Client(const std::string &host, uint16_t port, bool ssl,
// //          std::optional<std::string> auth)
// //       : host(host), port(port), ssl(ssl), auth(auth) {}

// //   std::string get(const std::string &path);
// //   std::string post_json(const std::string &path, const std::string &body);
// //   std::string post_form(const std::string &path, const std::string &body);

// //   void basic_auth(const std::string &user, const std::string &password);
// //   void jwt_auth(const std::string &token);

// //   static std::shared_ptr<Client> http(const std::string &host,
// //                                       uint16_t port = 80) {
// //     auto it = std::make_shared<Client>(host, port, false, std::nullopt);
// //     return it;
// //   }
// //   static std::shared_ptr<Client> https(const std::string &host,
// //                                        uint16_t port = 443) {
// //     auto it = std::make_shared<Client>(host, port, true, std::nullopt);
// //     return it;
// //   }

// //  private:
// //   HttpResponse handle(const boost::beast::http::verb &method,
// //                       const HttpRequest &request) const;

// //   inline static const std::string JSON = "application/json;charset=UTF-8";
// //   inline static const std::string FORM =
// //       "application/x-www-form-urlencoded;charset=UTF-8";

// //   std::string host;
// //   uint16_t port;
// //   bool ssl;
// //   std::optional<std::string> auth;
// // };

// // }  // namespace http

// // namespace http {
// // std::string get(const std::string &host, const std::string &target);
// // std::string get(const std::string &host, uint16_t port,
// //                 const std::string &target);
// // std::string post(const std::string &host, const std::string &target,
// //                  const std::string &form);
// // std::string post(const std::string &host, uint16_t port,
// //                  const std::string &target, const std::string &form);
// // }  // namespace http
// // namespace https {
// // std::string get(const std::string &host, const std::string &target);
// // std::string get(const std::string &host, uint16_t port,
// //                 const std::string &target);
// // std::string post(const std::string &host, const std::string &target,
// //                  const std::string &form);
// // std::string post(const std::string &host, uint16_t port,
// //                  const std::string &target, const std::string &form);
// // std::string post(const std::string &host, const std::string &user,
// //                  const std::string &password, const std::string &target,
// //                  const std::string &form);
// // std::string post(const std::string &host, uint16_t port,
// //                  const std::string &user, const std::string &password,
// //                  const std::string &target, const std::string &form);
// // std::string post(const std::string &host, uint16_t port,
// //                  std::optional<std::string> auth, const std::string
// &target,
// //                  const std::string &form);
// // }  // namespace https

// class Theme {
//  public:
//   Theme(const std::string &name)
//       : env(inja::Environment{std::filesystem::path("themes") / name}) {}

//   using Handler =
//       std::function<std::pair<std::filesystem::path, nlohmann::json>(
//           const httplib::Match &, const httplib::Params &)>;

//   template <class T>
//   std::string render(const T &page) {
//     return this->env.render_file(page.file(), page.data());
//   }

//  private:
//   inja::Environment env;
// };

// inline void log(const httplib::Result &res) {
//   if (res->status == 200 || res->status == 201) {
//     BOOST_LOG_TRIVIAL(debug) << res->status << " " << res->body;
//   } else {
//     BOOST_LOG_TRIVIAL(error) << res->status << " " << res->reason << "\n"
//                              << res->body;
//     for (auto it = res->headers.begin(); it != res->headers.end(); ++it) {
//       BOOST_LOG_TRIVIAL(debug) << it->first << ' ' << it->second;
//     }
//   }
// }
// // std::string render(const inja::Environment &env, std::filesystem::path
// &tpl,
// //                    const T &data) {
// //   return env.render_file(tpl, data);
// // }

// }  // namespace palm
