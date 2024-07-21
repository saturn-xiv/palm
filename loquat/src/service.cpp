#include "loquat/service.hpp"
#include "loquat/env.hpp"
#include "loquat/version.hpp"

#include <unistd.h>

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
#include <inja/inja.hpp>

void loquat::application::generate_systemd_config(const std::string& name,
                                                  const uint16_t port) {
  const std::string user = getlogin();
  const std::filesystem::path file(name + ".conf");
  spdlog::info("generate file {}", file.string());
  nlohmann::json data = {
      {"name", name},
      {"user", user},
      {"description", PROJECT_DESCRIPTION},
      {"port", port},
  };
  std::ofstream output(file);
  inja::render_to(output, R"PLAIN(
[Unit]
Description={{ description }}
Wants=network-online.target
After=network-online.target

[Service]
Type=notify
User={{ user }}
Group={{ user }}
ExecStart=/usr/bin/loquat rpc -p {{ port }}
WorkingDirectory=/var/lib/{{ name }}
Restart=always

[Install]
WantedBy=multi-user.target
)PLAIN",
                  data);
}

void loquat::application::launch_rpc_server(
    const uint16_t port, std::optional<loquat::application::Ssl> ssl) {
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

  const std::optional<std::string> jwt_id =
      request.__isset.jwt_id ? std::optional<std::string>{request.jwt_id}
                             : std::nullopt;
  const std::optional<std::string> key_id =
      request.__isset.key_id ? std::optional<std::string>{request.key_id}
                             : std::nullopt;
  const std::optional<std::string> payload =
      request.__isset.payload ? std::optional<std::string>{request.payload}
                              : std::nullopt;

  token = jwt.sign(jwt_id, key_id, request.issuer, request.subject,
                   request.audiences, absl::FromUnixSeconds(request.issued_at),
                   absl::FromUnixSeconds(request.not_before),
                   absl::FromUnixSeconds(request.expired_at), payload);
}

void loquat::JwtHandler::verify(loquat::v1::JwtVerfifyResponse& response,
                                const std::string& token,
                                const std::string& issuer,
                                const std::string& audience) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);

  loquat::Jwt jwt;
  const auto [jwt_id, key_id, subject, payload] =
      jwt.verify(token, issuer, audience);
  if (jwt_id) {
    response.__set_jwt_id(jwt_id.value());
  }
  if (key_id) {
    response.__set_key_id(key_id.value());
  }
  response.__set_subject(subject);
  if (payload) {
    response.__set_payload(payload.value());
  }
}

void loquat::HealthHandler::check(
    std::map<std::string, std::string>& response) {
  spdlog::info("call {}", __PRETTY_FUNCTION__);
  response["version"] = loquat::GIT_VERSION;
}
