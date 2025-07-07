#include <catch2/catch_test_macros.hpp>

#include "palm/rbac.hpp"

#include <iostream>

TEST_CASE("by PostgreSQL & RabbitMQ", "[casbin]") {
  spdlog::set_level(spdlog::level::debug);

  palm::PostgreSql pgsql("127.0.0.1", 5432, "www", "change-me", "lavender");
  auto pool = pgsql.open();

  palm::rabbitmq::Config rabbitmq;
  rabbitmq.set_virtual_host("vh.testing");
  rabbitmq.set_user("www");
  rabbitmq.set_password("change-me");
  const std::string WATCHER_CHANNEL = "casbin.watcher";

  std::shared_ptr<casbin::Watcher> watcher =
      std::make_shared<palm::casbin::RabbitMQWatcher>(
          "testing.casbin.watcher.worker", WATCHER_CHANNEL, rabbitmq);

  auto model = casbin::Model::NewModelFromString(palm::casbin::RBAC_MODEL);
  std::shared_ptr<casbin::Adapter> adapter =
      std::make_shared<palm::casbin::PostgreSQLAdapter>(pool);

  std::shared_ptr<casbin::Enforcer> enforcer =
      std::make_shared<casbin::Enforcer>(model, adapter);
  enforcer->EnableLog(true);
  enforcer->EnableAutoSave(true);
  enforcer->SetWatcher(watcher);
  enforcer->LoadPolicy();

  //   https://github.com/casbin/casbin-cpp/blob/master/examples/rbac_policy.csv
  SECTION("alice") {
    std::string sub = "alice";
    std::string obj = "data.1";
    std::string act = "read";

    REQUIRE(!enforcer->Enforce({sub, obj, act}));
    enforcer->AddPermissionForUser(sub, {obj, act});
    REQUIRE(enforcer->Enforce({sub, obj, act}));
    enforcer->DeletePermissionForUser(sub, {obj, act});
    REQUIRE(!enforcer->Enforce({sub, obj, act}));
  }
}

TEST_CASE("object/subject/permission", "[models]") {
  SECTION("user subject") {
    {
      int32_t id = 123;
      std::cout << "user by id(" << id << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::user::to_subject(id))
                << std::endl;
    }
    {
      std::string code = "abc";
      std::cout << "user by code(" << code << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::user::to_subject(code))
                << std::endl;
    }
  }

  SECTION("role subject") {
    std::cout << "role(root): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::role::root())
              << std::endl;
    std::cout << "role(administrator): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::role::administrator())
              << std::endl;
    {
      std::string code = "abc";
      std::cout << "role by code(" << code << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::role::other(code))
                << std::endl;
    }
  }

  SECTION("resource object") {
    const std::string type =
        boost::typeindex::type_id<palm::rabbitmq::Client>().pretty_name();
    std::cout << "resource(" << type << "): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::resource::to_object(type))
              << std::endl;
    {
      int32_t id = 123;
      std::cout << "resource by id(" << type << "," << id << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::resource::to_object(type, id))
                << std::endl;
    }
    {
      std::string code = "abc";
      std::cout << "resource by code(" << type << "," << code << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::resource::to_object(type, code))
                << std::endl;
    }
  }
  SECTION("action") {
    std::cout << "permission(read): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::read())
              << std::endl;
    std::cout << "permission(write): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::write())
              << std::endl;
    std::cout << "permission(append): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::append())
              << std::endl;
    std::cout << "permission(execute): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::execute())
              << std::endl;
    std::cout << "permission(debit): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::debit())
              << std::endl;
    std::cout << "permission(credit): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::credit())
              << std::endl;
    std::cout << "permission(inquiry): "
              << cppcodec::base64_url_unpadded::encode(
                     palm::casbin::permission::inquiry())
              << std::endl;
    {
      std::string code = "abc";
      std::cout << "permission by code(" << code << "): "
                << cppcodec::base64_url_unpadded::encode(
                       palm::casbin::permission::other(code))
                << std::endl;
    }
  }
}
