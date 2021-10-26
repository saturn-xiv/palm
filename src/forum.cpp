#include "palm/forum.hpp"

static void mount_topics(httplib::Server &svr) {
  svr.Get(R"(/api/forum/topics/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Post(R"(/api/forum/topics/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Delete(R"(/api/forum/topics/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Post("/api/forum/topics", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get("/api/forum/topics", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get(R"(/forum/topics/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Get("/forum/topics", [](const auto &req, auto &res) {
    //   TODO
  });
}

static void mount_posts(httplib::Server &svr) {
  svr.Get(R"(/api/forum/posts/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Post(R"(/api/forum/posts/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Delete(R"(/api/forum/posts/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Post("/api/forum/posts", [](const auto &req, auto &res) {
    //   TODO
  });
  svr.Get("/api/forum/posts", [](const auto &req, auto &res) {
    //   TODO
  });

  svr.Get(R"(/forum/posts/(\d+))", [&](const auto &req, auto &res) {
    auto id = req.matches[1];
    // TODO
  });
  svr.Get("/forum/posts", [](const auto &req, auto &res) {
    //   TODO
  });
}

void palm::forum::mount(httplib::Server &svr) {
  mount_topics(svr);
  mount_posts(svr);
  svr.Get("/forum", [](const auto &req, auto &res) {
    //   TODO
  });
}
