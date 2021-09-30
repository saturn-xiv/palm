#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

TEST_CASE("Demo", "[f]") { REQUIRE(2 * 2 == 4); }

// #define BOOST_TEST_MODULE zeromq
// #include <boost/test/included/unit_test.hpp>

// // #include "palm/crypto.hpp"
// // #include "palm/queue.hpp"
// // #include "palm/utils.hpp"

// // #include <unistd.h>
// // #include <ctime>
// // #include <sstream>

// // #include <boost/log/trivial.hpp>

// // const static uint16_t PORT = 10000;
// // const static std::string IPC = "/tmp/test.sck";
// // static std::vector<std::string> envelopes = {"q1", "q2", "q3"};

// BOOST_AUTO_TEST_CASE(publisher) {
//   // BOOST_LOG_TRIVIAL(info) << "test publisher";
//   // auto queue = std::make_shared<palm::zeromq::Publisher>(
//   //     palm::to_string(palm::zeromq::tcp(PORT)));

//   // for (auto i = 0; i < 100; i++) {
//   //   std::stringstream ss;
//   //   ss << "hi, zmq-pub!(" << i << ")";
//   //   const auto msg = ss.str();

//   //   queue->send(msg);
//   //   for (const auto& it : envelopes) {
//   //     queue->send(it, msg);

//   //     {
//   //       const std::time_t now = std::time(nullptr);
//   //       queue->send(it, std::asctime(std::localtime(&now)));
//   //     }
//   //   }

//   //   sleep(5);
//   // }
// }

// // class Subscriber : public palm::zeromq::Subscriber {
// //  public:
// //   Subscriber(const std::string& address,
// //              const std::vector<std::string>& envelopes)
// //       : palm::zeromq::Subscriber(address, envelopes) {
// //     this->id = palm::random::string(4);
// //   }

// //  protected:
// //   void handle(const std::optional<std::string>& envelope,
// //               const std::string& message) override {
// //     BOOST_LOG_TRIVIAL(info) << this->id << " handle message ("
// //                             << envelope.value() << ", " << message << ")";
// //   }

// //  private:
// //   std::string id;
// // };

// BOOST_AUTO_TEST_CASE(subscriber) {
//   // BOOST_LOG_TRIVIAL(info) << "test subscriber";
//   // Subscriber queue(palm::to_string(palm::zeromq::tcp("127.0.0.1", PORT)),
//   //                  envelopes);
//   // queue.start();
// }

// // class Consumer : public palm::zeromq::Consumer {
// //  public:
// //   Consumer(const std::string& address) : palm::zeromq::Consumer(address)
// {}

// //  protected:
// //   void handle(const std::string& message) override {
// //     BOOST_LOG_TRIVIAL(info) << "handle message (" << message << ")";
// //   }
// // };

// BOOST_AUTO_TEST_CASE(producer) {
//   // BOOST_LOG_TRIVIAL(info) << "test producer";
//   // palm::zeromq::Producer queue(palm::to_string(palm::zeromq::ipc(IPC)));
//   // const auto id = palm::random::string(4);
//   // for (auto i = 0; i < 100; i++) {
//   //   for (const auto& it : envelopes) {
//   //     sleep(5);
//   //     {
//   //       std::stringstream ss;
//   //       ss << "hi " << id << ", zmq push!(" << i << ")";
//   //       queue.send(ss.str());
//   //     }
//   //     sleep(5);
//   //     {
//   //       const std::time_t now = std::time(nullptr);
//   //       queue.send(std::asctime(std::localtime(&now)));
//   //     }
//   //   }
//   // }
// }

// BOOST_AUTO_TEST_CASE(consumer) {
//   // BOOST_LOG_TRIVIAL(info) << "test consumer";
//   // Consumer queue(palm::to_string(palm::zeromq::ipc(IPC)));
//   // queue.start();
// }
