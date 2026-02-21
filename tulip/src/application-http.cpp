#include "tulip/accounting.hpp"
#include "tulip/application.hpp"
#include "tulip/babel.hpp"
#include "tulip/blog.hpp"
#include "tulip/cms.hpp"
#include "tulip/forum.hpp"

#include <csignal>
#include <functional>

static std::function<void(int)> shutdown_handler;

static void signal_handler(int signal) { shutdown_handler(signal); }

int tulip::Application::http(const std::string& config_file, uint16_t port,
                             const std::string& theme) const {
  struct Config {
    Config(const toml::table& config)
        : postgresql(*(config["postgresql"].as_table())),
          redis(*(config["redis"].as_table())),
          rabbitmq(*(config["rabbitmq"].as_table())),
          opensearch(*(config["opensearch"].as_table())) {}

    palm::PostgreSql postgresql;
    palm::redis::Config redis;
    palm::rabbitmq::Config rabbitmq;
    palm::opensearch::Config opensearch;
    palm::grpc::Config daisy;
  };

  if (palm::is_stopped()) {
    return EXIT_SUCCESS;
  }

  const auto config_tree = toml::parse_file(config_file);
  Config config(config_tree);

  tulip::portal::Context ctx;
  ctx.db = config.postgresql.open();
  ctx.cache = config.redis.open();
  ctx.queue = config.rabbitmq.open();
  {
    spdlog::debug("load theme from {}", theme);
    ctx.env = std::make_shared<inja::Environment>(theme);
  }
  {
    spdlog::debug("open opensearch {}", config.opensearch.url(""));
    ctx.search = std::make_shared<palm::opensearch::Config>(config.opensearch);

    {
      const auto info = ctx.search->info();
      spdlog::debug("{} {} v{}", info->name, info->cluster_uuid,
                    info->version.number);
    }
  }

  const std::string host = "0.0.0.0";
  spdlog::info("start to listening on http://{}:{}", host, port);
  httplib::Server server;

  server.Get("/cms/pages", [&ctx](const httplib::Request& req,
                                  httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    auto page = tulip::portal::page(req);
    const auto data = tulip::cms::controllers::pages::index(ctx, ss, page);
    palm::http::html(res, ctx.env, "cms/pages/index.html", *data);
  });
  server.Get("/cms/pages/:permalink",
             [&ctx](const httplib::Request& req, httplib::Response& res) {
               auto ss = tulip::portal::session(req);
               const auto data = tulip::cms::controllers::pages::show(
                   ctx, ss, req.path_params.at("permalink"));
               palm::http::html(res, ctx.env, "cms/pages/show.html", *data);
             });
  server.Get("/api/cms/pages", [&ctx](const httplib::Request& req,
                                      httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    palm::portal::v1::Page page;
    if (!palm::http::body(req, res, &page)) {
      return;
    }
    const auto body = tulip::cms::controllers::pages::index(ctx, ss, page);
    palm::http::json(res, *body);
  });
  server.Get("/api/cms/pages/:id", [&ctx](const httplib::Request& req,
                                          httplib::Response& res) {
    auto ss = tulip::portal::session(req);
    palm::portal::v1::IdRequest req_;
    if (!palm::http::body(req, res, &req_)) {
      return;
    }
    const auto body = tulip::cms::controllers::pages::show(ctx, ss, req_);
    palm::http::json(res, *body);
  });

  {
    std::map<std::string, std::string> items = {{"/statics", "./assets"},
                                                {"/3rd", "./node_modules"}};
    for (auto const& [key, val] : items) {
      spdlog::debug("mount assets folder {}=>{}", val, key);
      auto ret = server.set_mount_point(key, val);
      if (!ret) {
        spdlog::error("couldn't mount {}", key);
      }
    }
  }

  server.set_payload_max_length(1024 * 1024 * 5);
  server.set_logger(
      [](const httplib::Request& req, const httplib::Response& res) {
        spdlog::info("{} {} {}", req.method, req.path, res.status);
      });
  server.set_exception_handler(
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

  shutdown_handler = [&](int signal) {
    if (signal == SIGINT) {
      spdlog::warn("Ctrl+C caught, exiting...");
      server.stop();
    }
  };
  std::signal(SIGINT, signal_handler);
  server.listen(host, port);
  return EXIT_SUCCESS;
}
