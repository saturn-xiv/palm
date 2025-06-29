#define BOOST_TEST_MODULE rpc
#include <boost/test/included/unit_test.hpp>

#include "basil/wisteria.hpp"

BOOST_AUTO_TEST_CASE(grpc_) {
  basil::GrpcClient cfg("127.0.0.1", 8080);
  auto channel =
      grpc::CreateChannel(cfg.target(), grpc::InsecureChannelCredentials());
  {
    basil::wisteria::rpc::UserClient client(channel);
    auto response = client.sign_in("guest", "guest");
    std::cout << "token: " << response->token() << std::endl;
  }
  BOOST_CHECK_EQUAL(1 + 1, 2);
}
