#include <catch2/catch_test_macros.hpp>

#include "palm/rbac.hpp"

#include <iostream>

TEST_CASE("by PostgreSQL & RabbitMQ", "[casbin]") {}

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
