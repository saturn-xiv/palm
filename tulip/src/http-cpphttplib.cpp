#include "tulip/accounting.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"
#include "tulip/http.hpp"

void tulip::http::cpp_httplib::Server::mount(
    tulip::portal::Context& ctx,
    const std::map<std::string, std::filesystem::path>& assets) {
  this->_server->Get("/cms/pages", [&ctx](const httplib::Request& req,
                                          httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    auto page = tulip::portal::page(req);
    const auto data = tulip::cms::controllers::pages::index(ctx, ss, page);
    palm::http::html(res, ctx.env, "cms/pages/index.html", *data);
  });
  this->_server->Get(
      "/cms/pages/:permalink",
      [&ctx](const httplib::Request& req, httplib::Response& res) {
        auto ss = tulip::portal::session(req);
        const auto data = tulip::cms::controllers::pages::show(
            ctx, ss, req.path_params.at("permalink"));
        palm::http::html(res, ctx.env, "cms/pages/show.html", *data);
      });
  this->_server->Get("/api/cms/pages", [&ctx](const httplib::Request& req,
                                              httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    palm::portal::v1::Page page;
    if (!palm::http::body(req, res, &page)) {
      return;
    }
    const auto body = tulip::cms::controllers::pages::index(ctx, ss, page);
    palm::http::json(res, *body);
  });
  this->_server->Get("/api/cms/pages/:id", [&ctx](const httplib::Request& req,
                                                  httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    palm::portal::v1::IdRequest req_;
    if (!palm::http::body(req, res, &req_)) {
      return;
    }
    const auto body = tulip::cms::controllers::pages::show(ctx, ss, req_);
    palm::http::json(res, *body);
  });

  for (auto const& [key, val] : assets) {
    spdlog::debug("mount assets folder {}=>{}", val.string(), key);
    auto ret = this->_server->set_mount_point(key, val);
    if (!ret) {
      spdlog::error("couldn't mount {}", key);
    }
  }

  this->_server->set_payload_max_length(1024 * 1024 * 5);
  this->_server->set_logger(
      [](const httplib::Request& req, const httplib::Response& res) {
        spdlog::info("{} {} {}", req.method, req.path, res.status);
      });
  this->_server->set_exception_handler(
      [](const auto& req, auto& res, std::exception_ptr err) {
        try {
          std::rethrow_exception(err);
        } catch (std::exception& e) {
          palm::http::text(res, httplib::StatusCode::InternalServerError_500,
                           e.what());
        } catch (...) {
          palm::http::text(res, httplib::StatusCode::InternalServerError_500,
                           "Unknown exception");
        }
      });
}
