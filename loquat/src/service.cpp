#include "loquat/service.hpp"
#include "loquat/env.hpp"

#include <thrift/concurrency/ThreadFactory.h>
#include <thrift/concurrency/ThreadManager.h>
#include <thrift/processor/TMultiplexedProcessor.h>
#include <thrift/protocol/TBinaryProtocol.h>
#include <thrift/server/TNonblockingServer.h>
#include <thrift/server/TThreadPoolServer.h>
#include <thrift/server/TThreadedServer.h>
#include <thrift/transport/TNonblockingSSLServerSocket.h>
#include <thrift/transport/TNonblockingServerSocket.h>
#include <thrift/transport/TSSLServerSocket.h>
#include <thrift/transport/TSSLSocket.h>
#include <thrift/transport/TServerSocket.h>
#include <thrift/transport/TSocket.h>
#include <thrift/transport/TTransportUtils.h>

void loquat::application::launch(const uint16_t port,
                                 std::optional<loquat::application::Ssl> ssl) {
  std::shared_ptr<AesHandler> aesHandler = std::make_shared<AesHandler>();
  std::shared_ptr<v1::AesProcessor> aesProcessor =
      std::make_shared<v1::AesProcessor>(aesHandler);

  std::shared_ptr<HmacHandler> hmacHandler = std::make_shared<HmacHandler>();
  std::shared_ptr<v1::HmacProcessor> hmacProcessor =
      std::make_shared<v1::HmacProcessor>(hmacHandler);

  std::shared_ptr<JwtHandler> jwtHandler = std::make_shared<JwtHandler>();
  std::shared_ptr<v1::JwtProcessor> jwtProcessor =
      std::make_shared<v1::JwtProcessor>(jwtHandler);

  std::shared_ptr<HealthHandler> healthHandler =
      std::make_shared<HealthHandler>();
  std::shared_ptr<v1::HealthProcessor> healthProcessor =
      std::make_shared<v1::HealthProcessor>(healthHandler);

  std::shared_ptr<apache::thrift::TMultiplexedProcessor> multiplexedProcessor =
      std::make_shared<apache::thrift::TMultiplexedProcessor>();

  {
    const auto name = typeid(v1::AesIf).name();
    spdlog::info("register aes service {}", name);
    multiplexedProcessor->registerProcessor(name, aesProcessor);
  }
  {
    const auto name = typeid(v1::HmacIf).name();
    spdlog::info("register hmac service {}", name);
    multiplexedProcessor->registerProcessor(name, hmacProcessor);
  }
  {
    const auto name = typeid(v1::JwtIf).name();
    spdlog::info("register jwt service {}", name);
    multiplexedProcessor->registerProcessor(name, jwtProcessor);
  }
  {
    const auto name = typeid(v1::HealthIf).name();
    spdlog::info("register health service {}", name);
    multiplexedProcessor->registerProcessor(name, healthProcessor);
  }

  std::shared_ptr<apache::thrift::TProcessor> processor =
      std::dynamic_pointer_cast<apache::thrift::TProcessor>(
          multiplexedProcessor);

  auto threads_count = std::thread::hardware_concurrency();

  std::shared_ptr<apache::thrift::concurrency::ThreadFactory> threadFactory =
      std::make_shared<apache::thrift::concurrency::ThreadFactory>();
  std::shared_ptr<apache::thrift::concurrency::ThreadManager> threadManager =
      apache::thrift::concurrency::ThreadManager::newSimpleThreadManager(
          threads_count * 4 + 1);
  threadManager->threadFactory(threadFactory);
  threadManager->start();

  std::shared_ptr<apache::thrift::protocol::TBinaryProtocolFactoryT<
      apache::thrift::transport::TFramedTransport>>
      protocolFactory =
          std::make_shared<apache::thrift::protocol::TBinaryProtocolFactoryT<
              apache::thrift::transport::TFramedTransport>>();

  std::shared_ptr<apache::thrift::server::TNonblockingServer> server;

  if (ssl) {
    std::shared_ptr<apache::thrift::transport::TSSLSocketFactory>
        sslSocketFactory =
            std::make_shared<apache::thrift::transport::TSSLSocketFactory>();
    {
      spdlog::info("listening on tcps://0.0.0.0:{}", port);
      spdlog::debug("load cert from {}, key from {}, ca from {}",
                    ssl->cert_file, ssl->key_file, ssl->ca_file);
      sslSocketFactory->loadCertificate(ssl->cert_file.c_str());
      sslSocketFactory->loadPrivateKey(ssl->key_file.c_str());
      sslSocketFactory->loadTrustedCertificates(ssl->ca_file.c_str());

      sslSocketFactory->ciphers("ALL:!ADH:!LOW:!EXP:!MD5:@STRENGTH");
    }

    std::shared_ptr<apache::thrift::transport::TNonblockingSSLServerSocket>
        serverSocket = std::make_shared<
            apache::thrift::transport::TNonblockingSSLServerSocket>(
            port, sslSocketFactory);

    server = std::make_shared<apache::thrift::server::TNonblockingServer>(
        multiplexedProcessor, protocolFactory, serverSocket, threadManager);
  } else {
    spdlog::info("listening on tcp://0.0.0.0:{}", port);
    std::shared_ptr<apache::thrift::transport::TNonblockingServerSocket>
        serverSocket = std::make_shared<
            apache::thrift::transport::TNonblockingServerSocket>(port);
    server = std::make_shared<apache::thrift::server::TNonblockingServer>(
        multiplexedProcessor, protocolFactory, serverSocket, threadManager);
  }

  server->setNumIOThreads(threads_count * 2 + 1);
  server->serve();
}

void loquat::AesHandler::encrypt(std::string& code, const std::string& plain) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::Aes aes;
  code = aes.encrypt(plain);
}

void loquat::AesHandler::decrypt(std::string& plain, const std::string& code) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::Aes aes;
  plain = aes.decrypt(code);
}

void loquat::HmacHandler::sign(std::string& code, const std::string& plain) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::HMac mac;
  code = mac.sign(plain);
}

void loquat::HmacHandler::verify(const std::string& code,
                                 const std::string& plain) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::HMac mac;
  mac.verify(code, plain);
}

void loquat::JwtHandler::sign(std::string& token,
                              const loquat::v1::JwtSignRequest& request) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::Jwt jwt;
  const auto ttl = std::chrono::seconds(request.ttl);
  if (request.__isset.payload) {
  }
  const std::optional<std::string> payload =
      request.__isset.payload ? std::optional<std::string>{request.payload}
                              : std::nullopt;
  token = jwt.sign(request.issuer, request.subject, request.audiences, ttl,
                   payload);
}

void loquat::JwtHandler::verify(loquat::v1::JwtVerfifyResponse& response,
                                const std::string& token,
                                const std::string& issuer,
                                const std::string& audience) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  loquat::Jwt jwt;
  const auto it = jwt.verify(token, issuer, audience);
  response.subject = it.first;
  if (it.second) {
    response.payload = it.second.value();
  }
}

void loquat::HealthHandler::check() {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
}
