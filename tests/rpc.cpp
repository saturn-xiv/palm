#include <catch2/catch_test_macros.hpp>

TEST_CASE("grpc client", "[grpc]") {
  // palm::GrpcClient cfg("127.0.0.1", 8080);
  // auto channel =
  //     grpc::CreateChannel(cfg.target(), grpc::InsecureChannelCredentials());
  // {
  //   palm::portal::rpc::UserClient client(channel);
  //   auto response = client.sign_in("guest", "guest");
  //   std::cout << "token: " << response->token() << std::endl;
  // }
}
