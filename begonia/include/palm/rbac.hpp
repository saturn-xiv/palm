#pragma once

#include "casbin.grpc.pb.h"
#include "palm/orm.hpp"
#include "palm/queue.hpp"

#include <boost/type_index.hpp>

#include <casbin/casbin.h>
#include <google/protobuf/arena.h>
#include <cppcodec/base64_url_unpadded.hpp>

namespace palm {
namespace casbin {
// https://github.com/casbin/casbin/blob/master/examples/rbac_model.conf
inline static const std::string RBAC_MODEL = R"RBAC(
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
)RBAC";

// https://github.com/casbin/gorm-adapter-ex
class PostgreSQLAdapter : public ::casbin::Adapter {
 public:
  PostgreSQLAdapter(std::shared_ptr<soci::connection_pool> pool)
      : _pool(pool), _is_filtered(false) {}

  void LoadPolicy(const std::shared_ptr<::casbin::Model>& model) override;
  void SavePolicy(const std::shared_ptr<::casbin::Model>& model) override;
  void AddPolicy(std::string sec, std::string p_type,
                 std::vector<std::string> rule) override;
  void RemovePolicy(std::string sec, std::string p_type,
                    std::vector<std::string> rule) override;
  void RemoveFilteredPolicy(std::string sec, std::string ptype, int field_index,
                            std::vector<std::string> field_values) override;
  bool IsFiltered() override;
  bool IsValid() override;

 private:
  std::shared_ptr<soci::connection_pool> _pool;
  bool _is_filtered;
};

// https://github.com/casbin/redis-watcher
class RabbitMQWatcher : public ::casbin::Watcher {
 public:
  RabbitMQWatcher(const std::string& local_id, const std::string& channel,
                  const palm::rabbitmq::Config& queue)
      : _local_id(local_id),
        _channel(channel),
        _publisher(queue.open()),
        _subscriber(queue.open()) {}

  void Update() override;
  void Close() override;

  void subscribe(std::shared_ptr<::casbin::Enforcer> enforcer);

 private:
  std::string _local_id;
  std::string _channel;
  std::shared_ptr<palm::rabbitmq::Client> _publisher;
  std::shared_ptr<palm::rabbitmq::Client> _subscriber;
};

namespace user {
std::string to_subject(int32_t id);
std::string to_subject(const std::string& code);
}  // namespace user

namespace role {
std::string root();
std::string administrator();
std::string other(const std::string& code);
}  // namespace role

namespace permission {
std::string read();
std::string write();
std::string append();
std::string execute();
std::string credit();
std::string debit();
std::string inquiry();
std::string other(const std::string& code);
}  // namespace permission

namespace resource {
std::string to_object(const std::string& type, int32_t id);
std::string to_object(const std::string& type, const std::string& code);
std::string to_object(const std::string& type);
}  // namespace resource

}  // namespace casbin
}  // namespace palm
