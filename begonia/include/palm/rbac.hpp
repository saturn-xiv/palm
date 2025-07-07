#pragma once

#include "palm/orm.hpp"
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
class PostgreSQLAdapter {};
// https://github.com/casbin/redis-watcher
class RabbitMQWatcher {};
}  // namespace casbin
}  // namespace palm
