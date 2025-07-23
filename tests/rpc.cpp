#include <catch2/catch_test_macros.hpp>

#include "monitoring.grpc.pb.h"
#include "palm/rpc.hpp"
#include "portal.grpc.pb.h"

#include <google/protobuf/util/json_util.h>
#include <google/protobuf/util/time_util.h>

TEST_CASE("phlox client", "[grpc]") {
  palm::GRpcClient config("127.0.0.0", 18080);
  auto channel = config.open();
}

TEST_CASE("protobuf map", "[grpc]") {
  // FIXME clang-20 segmentation fault (core dumped)
  SECTION("std") {
    palm::monitoring::v1::PodmanContainersResponse_Item it;
    {
      auto x = it.mutable_labels();
      (*x)["111"] = "abc";
      (*x)["222"] = "xyz";
    }
    {
      std::cout << "map size " << it.labels().size() << std::endl;
      for (const auto& [k, v] : it.labels()) {
        std::cout << k << "=" << v << std::endl;
      }
    }
    std::string buf;

    const auto status = google::protobuf::util::MessageToJsonString(it, &buf);
    REQUIRE(status.ok());
    std::cout << "map(std): " << buf << std::endl;
  }
  SECTION("arena") {
    google::protobuf::Arena arena;
    auto it = google::protobuf::Arena::Create<
        palm::monitoring::v1::PodmanContainersResponse_Item>(&arena);
    {
      auto x = it->mutable_labels();
      (*x)["111"] = "abc";
      (*x)["222"] = "xyz";
    }

    std::string buf;
    const auto status = google::protobuf::util::MessageToJsonString(*it, &buf);
    REQUIRE(status.ok());
    std::cout << "map(arena): " << buf << std::endl;
  }
}

TEST_CASE("grpc client", "[grpc]") {
  google::protobuf::Arena arena;

  SECTION("tm2ts") {
    {
      const auto now = std::chrono::system_clock::now();

      {
        const time_t seconds = std::chrono::system_clock::to_time_t(now);
        std::cout << std::ctime(&seconds) << std::endl;
        google::protobuf::Timestamp* it =
            google::protobuf::Arena::Create<google::protobuf::Timestamp>(
                &arena);
        it->set_seconds(seconds);
        it->set_nanos(0);

        std::string buf;
        const auto status =
            google::protobuf::util::MessageToJsonString(*it, &buf);
        REQUIRE(status.ok());
        std::cout << "utc now(system_clock): " << buf << std::endl;
      }
    }

    {
      const auto now = std::chrono::high_resolution_clock::now();
      std::cout << "utc now(high_resolution_clock nano): "
                << now.time_since_epoch().count() << std::endl;
      {
        // TODO
        // google::protobuf::Timestamp* it =
        //     google::protobuf::Arena::Create<google::protobuf::Timestamp>(
        //         &arena);
        // it->set_seconds(seconds);
        // it->set_nanos(0);

        // std::string buf;
        // const auto status =
        //     google::protobuf::util::MessageToJsonString(*it, &buf);
        // REQUIRE(status.ok());
        // std::cout << "utc now(high_resolution_clock): " << buf << std::endl;
      }
    }
  }

  SECTION("ts2tm") {
    const auto now = google::protobuf::util::TimeUtil::GetCurrentTime();
    {
      std::string buf;
      const auto status =
          google::protobuf::util::MessageToJsonString(now, &buf);
      REQUIRE(status.ok());
      std::cout << "utc now(protobuf util): " << buf << std::endl;
    }
    {
      time_t seconds = now.seconds();
      std::tm* it = std::gmtime(&seconds);
      std::cout << "asc time: " << std::asctime(it) << std::endl;
    }
  }

  SECTION("to_json") {
    palm::portal::v1::UserSignInByEmailRequest* req =
        google::protobuf::Arena::Create<
            palm::portal::v1::UserSignInByEmailRequest>(&arena);
    {
      req->set_email("who-ami-i@local");
      req->set_password("change-me");
      google::protobuf::Duration* ttl =
          google::protobuf::Arena::Create<google::protobuf::Duration>(&arena);
      ttl->set_seconds(123456);
      ttl->set_nanos(321);
      req->set_allocated_ttl(ttl);
    }

    std::string buf;
    {
      const auto status =
          google::protobuf::util::MessageToJsonString(*req, &buf);
      REQUIRE(status.ok());
      std::cout << "UserSignInByEmailRequest: " << buf << std::endl;
    }

    {
      palm::portal::v1::UserSignInByEmailRequest* tmp =
          google::protobuf::Arena::Create<
              palm::portal::v1::UserSignInByEmailRequest>(&arena);
      const auto status = google::protobuf::util::JsonStringToMessage(buf, tmp);
      REQUIRE(status.ok());
      REQUIRE(tmp->email() == req->email());
      REQUIRE(tmp->password() == req->password());
      REQUIRE(tmp->has_ttl());
      REQUIRE(tmp->ttl().seconds() == req->ttl().seconds());
      REQUIRE(tmp->ttl().nanos() == req->ttl().nanos());
    }
  }
  // palm::GrpcClient cfg("127.0.0.1", 8080);
  // auto channel =
  //     grpc::CreateChannel(cfg.target(), grpc::InsecureChannelCredentials());
  // {
  //   palm::portal::rpc::UserClient client(channel);
  //   auto response = client.sign_in("guest", "guest");
  //   std::cout << "token: " << response->token() << std::endl;
  // }
}
