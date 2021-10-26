#include "palm/nut.hpp"

static void mount_admin(httplib::Server &svr) {}
static void mount_home(httplib::Server &svr) {
  {
    auto ret = svr.set_mount_point("/3rd", "./node_modules");
    if (!ret) {
      BOOST_LOG_TRIVIAL(error) << "can't find node_modules folder";
    }
  }

  svr.Get("/robots.txt", [](const auto &req, auto &res) {
    const std::string tpl = R"(User-agent: *
Disallow: /api/
Sitemap: https://{{ domain }}/sitemap.xml.gz)";

    //   TODO
  });
  svr.Get(R"(/rss/([-_\w]+)\.xml)", [&](const auto &req, auto &res) {
    auto lang = req.matches[1];
    //   TODO
  });
  svr.Get("/sitemap.xml.gz", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get("/", [](const auto &req, auto &res) {
    //   TODO
  });
}
static void setup_http_server(httplib::Server &svr) {
  svr.set_logger([](const auto &req, const auto &res) {
    //   TDOD
  });
  svr.set_error_handler([](const auto &req, auto &res) {
    // TODO
  });
  svr.set_exception_handler([](const auto &req, auto &res, std::exception &e) {
    res.status = 500;
    // TODO
  });
}
static void mount_attachments(httplib::Server &svr) {
  svr.Post("/api/attachments", [&](const auto &req, auto &res) {
    // TODO
    //   auto size = req.files.size();
    //   auto ret = req.has_file("name1");
    //   const auto& file = req.get_file_value("name1");
    // file.filename;
    // file.content_type;
    // file.content;
  });
}
static void mount_leave_words(httplib::Server &svr) {
  svr.Delete(R"(/api/leave-words/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO admin
  });
  svr.Post("/api/leave-words", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get("/api/leave-words", [](const auto &req, auto &res) {
    //   TODO
  });
}

static void mount_users(httplib::Server &svr) {
  svr.Post("/api/users/sign-in", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Post("/api/users/sign-up", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Post("/api/users/confirm", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get(R"(/api/users/confirm/([-_\w]+))", [&](const auto &req, auto &res) {
    auto numbers = req.matches[1];
    //   TODO
  });
  svr.Post("/api/users/unlock", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get(R"(/api/users/unlock/([-_\w]+))", [&](const auto &req, auto &res) {
    auto numbers = req.matches[1];
    //   TODO
  });
  svr.Delete("/api/users/sign-out", [](const auto &req, auto &res) {
    //   TODO
  });
}

void palm::nut::mount(httplib::Server &svr) {
  mount_users(svr);
  mount_leave_words(svr);
  mount_attachments(svr);
  mount_admin(svr);
  mount_home(svr);
  setup_http_server(svr);
}
