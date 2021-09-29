#define BOOST_TEST_MODULE rabbitmq
#include <boost/test/included/unit_test.hpp>

// #include "palm/queue.hpp"

// #define PALM_QUEUE "test"

// void open(std::function<void(std::shared_ptr<palm::rabbitmq::Connection>)>
// fn) {
//   toml::table cfg;

//   cfg.insert("pool-size", 5);
//   cfg.insert("virtual-host", "/demo");
//   std::cout << cfg << std::endl;

//   auto builder = std::make_shared<palm::rabbitmq::Config>();
//   *builder = cfg;
//   auto pool = std::make_shared<palm::rabbitmq::Pool>(builder);

//   auto it = pool->get();
//   palm::rabbitmq::PooledConnection con(pool, it);

//   std::cout << "### into fun fun ### " << (it == nullptr) << std::endl;
//   fn(it);
//   std::cout << "### out from fun ### " << (it == nullptr) << std::endl;
// }

BOOST_AUTO_TEST_CASE(pub) {
  // open([](std::shared_ptr<palm::rabbitmq::Connection> it) {
  //   for (int i = 1; i < 10; i++) {
  //     std::stringstream ss;
  //     ss << "hello, " << i << ".";
  //     it->publish(PALM_QUEUE, "plain", ss.str());
  //   }
  // });
}

// class Handler : public palm::rabbitmq::Handler {
//  public:
//   void execute(const std::string& id, const std::string& content_type,
//                const std::string& body) override {
//     std::cout << "process message " << id << "@" << content_type << "\n"
//               << body << std::endl;
//   }
// };

BOOST_AUTO_TEST_CASE(sub) {
  // auto handler = std::make_shared<Handler>();
  // open([&](std::shared_ptr<palm::rabbitmq::Connection> it) {
  //   std::cout << "### sub 1 ### " << (it == nullptr) << std::endl;
  //   it->consume(PALM_QUEUE, handler);
  // });
}
