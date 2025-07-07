#include <catch2/catch_test_macros.hpp>

#include "palm/http.hpp"
#include "palm/queue.hpp"

#include <format>
#include <iostream>
#include <thread>

class EchoQueueConsumer final : public palm::QueueConsumer {
 public:
  EchoQueueConsumer(const std::string& name) : _name(name) {}
  std::string name() override { return this->_name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override {
    std::string msg(payload.begin(), payload.end());
    std::cout << "echo: " << msg << std::endl;
  }

 private:
  std::string _name;
};

TEST_CASE("by rabbitmq", "[rabbitmq]") {
  const std::string PRODUCER_CONSUMER_QUEUE = "qu.p-c";
  const std::string PUBLISHER_SUBSCRIBER_EXCHANGE = "ex.pub-sub";

  palm::rabbitmq::Config cfg;
  cfg.set_virtual_host("vh.testing");
  cfg.set_user("www");
  cfg.set_password("change-me");

  auto cli = cfg.open();
  REQUIRE(cli != nullptr);

  SECTION("producer") {
    std::chrono::seconds span(2);
    for (int i = 0;; i++) {
      const std::string msg = std::format("message {} from producer", i);
      std::cout << "produce " << msg << std::endl;
      std::vector<uint8_t> payload(msg.begin(), msg.end());
      cli->produce(PRODUCER_CONSUMER_QUEUE,
                   palm::http::content_type::TEXT_PLAIN_UTF8, payload);
      std::this_thread::sleep_for(span);
    }
  }

  SECTION("consumer") {
    std::shared_ptr<palm::QueueConsumer> consumer =
        std::make_shared<EchoQueueConsumer>("echo.consumer");
    cli->consume(PRODUCER_CONSUMER_QUEUE, consumer);
  }

  SECTION("publisher") {
    std::chrono::seconds span(2);
    for (int i = 0;; i++) {
      const std::string msg = std::format("message {} from publisher", i);
      std::cout << "publish " << msg << std::endl;
      std::vector<uint8_t> payload(msg.begin(), msg.end());
      cli->publish(PUBLISHER_SUBSCRIBER_EXCHANGE,
                   palm::http::content_type::TEXT_PLAIN_UTF8, payload);
      std::this_thread::sleep_for(span);
    }
  }

  SECTION("subscriber") {
    std::shared_ptr<palm::QueueConsumer> consumer =
        std::make_shared<EchoQueueConsumer>("echo.subscriber");
    cli->subscribe(PUBLISHER_SUBSCRIBER_EXCHANGE, consumer);
  }
}
