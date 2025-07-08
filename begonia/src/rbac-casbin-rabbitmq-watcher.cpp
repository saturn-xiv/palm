#include "palm/http.hpp"
#include "palm/rbac.hpp"

#include <unistd.h>

#include <boost/asio/ip/host_name.hpp>

namespace palm {
namespace casbin {
class WatcherConsumer final : public palm::QueueConsumer {
 public:
  WatcherConsumer(const std::string& name, const std::string& local_id,
                  std::shared_ptr<::casbin::Enforcer> enforcer)
      : _name(name), _local_id(local_id), _enforcer(enforcer) {}
  std::string name() override { return this->_name; }
  void execute(const std::string& id, const std::string& content_type,
               const std::vector<uint8_t> payload) override {
    if (content_type != palm::http::content_type::APPLICATION_X_PROTOBUF) {
      spdlog::warn("ignore message ({}, {})", id, content_type);
      return;
    }
    palm::casbin::v1::WatcherMessage message;
    if (!message.ParseFromArray(payload.data(), payload.size())) {
      spdlog::error("failed to parse message");
      return;
    }
    if (message.id() == this->_local_id) {
      spdlog::debug("ignore refresh message");
      return;
    }
    spdlog::debug("reload casbin policies");
    this->_enforcer->LoadPolicy();
  }

 private:
  std::string _name;
  std::string _local_id;
  std::shared_ptr<::casbin::Enforcer> _enforcer;
};
}  // namespace casbin
}  // namespace palm

void palm::casbin::RabbitMQWatcher::subscribe(
    std::shared_ptr<::casbin::Enforcer> enforcer) {
  std::shared_ptr<palm::casbin::WatcherConsumer> consumer =
      std::make_shared<palm::casbin::WatcherConsumer>(
          std::format("casbin-watcher.{}-{}", boost::asio::ip::host_name(),
                      getpid()),
          this->_local_id, enforcer);
  this->_subscriber->subscribe(this->_channel, consumer);
}

void palm::casbin::RabbitMQWatcher::Update() {
  std::vector<uint8_t> buffer;
  {
    google::protobuf::Arena arena;
    palm::casbin::v1::WatcherMessage* message =
        google::protobuf::Arena::Create<palm::casbin::v1::WatcherMessage>(
            &arena);
    message->set_id(this->_local_id);
    message->set_method(palm::casbin::v1::WatcherMessage_Method_Update);

    const auto len = message->ByteSizeLong();
    buffer.reserve(len);
    if (!message->SerializeToArray(buffer.data(), len)) {
      spdlog::error("failed to serial casbin watcher message");
      return;
    }
  }
  this->_publisher->publish(
      this->_channel, palm::http::content_type::APPLICATION_X_PROTOBUF, buffer);
}
void palm::casbin::RabbitMQWatcher::Close() {
  spdlog::warn("casbin watcher({}) exit", this->_local_id);
}
