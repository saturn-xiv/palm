#define BOOST_TEST_MODULE queue
#include <boost/test/included/unit_test.hpp>

#include "palm/queue.hpp"

#include <format>
#include <thread>

#define RABBITMQ_VIRTUAL_HOST "vh.testing"
#define RABBITMQ_VIRTUAL_USER "www"
#define RABBITMQ_VIRTUAL_PASSWORD "change-me"
#define RABBITMQ_PRODUCER_CONSUMER_QUEUE "qu.p-c"
#define RABBITMQ_PUBLISHER_SUBSCRIBER_EXCHANGE "ex.pub-sub"
#define RABBITMQ_CONTENT_TYPE "text/plain"

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

BOOST_AUTO_TEST_CASE(rabbitmq_producer) {
  palm::rabbitmq::Config cfg;
  cfg.set_virtual_host(RABBITMQ_VIRTUAL_HOST);
  cfg.set_user(RABBITMQ_VIRTUAL_USER);
  cfg.set_password(RABBITMQ_VIRTUAL_PASSWORD);

  auto cli = cfg.open();
  BOOST_REQUIRE(cli);
  std::chrono::seconds span(2);
  for (int i = 0;; i++) {
    const std::string msg = std::format("message {} from producer", i);
    std::cout << "produce " << msg << std::endl;
    std::vector<uint8_t> payload(msg.begin(), msg.end());
    cli->produce(RABBITMQ_PRODUCER_CONSUMER_QUEUE, RABBITMQ_CONTENT_TYPE,
                 payload);
    std::this_thread::sleep_for(span);
  }
}
BOOST_AUTO_TEST_CASE(rabbitmq_consumer) {
  palm::rabbitmq::Config cfg;
  cfg.set_virtual_host(RABBITMQ_VIRTUAL_HOST);
  cfg.set_user(RABBITMQ_VIRTUAL_USER);
  cfg.set_password(RABBITMQ_VIRTUAL_PASSWORD);
  auto cli = cfg.open();
  BOOST_REQUIRE(cli);

  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<EchoQueueConsumer>("echo.consumer");
  cli->consume(RABBITMQ_PRODUCER_CONSUMER_QUEUE, consumer);
}

BOOST_AUTO_TEST_CASE(rabbitmq_publisher) {
  palm::rabbitmq::Config cfg;
  cfg.set_virtual_host(RABBITMQ_VIRTUAL_HOST);
  cfg.set_user(RABBITMQ_VIRTUAL_USER);
  cfg.set_password(RABBITMQ_VIRTUAL_PASSWORD);
  auto cli = cfg.open();
  BOOST_REQUIRE(cli);

  std::chrono::seconds span(2);
  for (int i = 0;; i++) {
    const std::string msg = std::format("message {} from publisher", i);
    std::cout << "publish " << msg << std::endl;
    std::vector<uint8_t> payload(msg.begin(), msg.end());
    cli->publish(RABBITMQ_PUBLISHER_SUBSCRIBER_EXCHANGE, RABBITMQ_CONTENT_TYPE,
                 payload);
    std::this_thread::sleep_for(span);
  }
}
BOOST_AUTO_TEST_CASE(rabbitmq_subscriber) {
  palm::rabbitmq::Config cfg;
  cfg.set_virtual_host(RABBITMQ_VIRTUAL_HOST);
  cfg.set_user(RABBITMQ_VIRTUAL_USER);
  cfg.set_password(RABBITMQ_VIRTUAL_PASSWORD);
  auto cli = cfg.open();
  BOOST_REQUIRE(cli);

  std::shared_ptr<palm::QueueConsumer> consumer =
      std::make_shared<EchoQueueConsumer>("echo.subscriber");
  cli->subscribe(RABBITMQ_PUBLISHER_SUBSCRIBER_EXCHANGE, consumer);
}
