namespace cpp gourd.v1
namespace java com.github.saturn_xiv.palm.plugins.gourd.v1

const string ROLE_ADMINISTRATOR = "administrator"
const string ROLE_ROOT = "root"

struct User{
    1:i64 id,
}

struct Role{
    1:string name,
}

struct Resource{
    1:string type,
    2:optional i64 id,
}

struct Permission{
    1:string operation,
    2:Resource resource,
}


service Policy {
    bool has(1:i64 user, 2:string role);
    bool can(1:i64 user, 2:string operation, 3:Resource resource);
    void delete_user(1:i64 user);
    void delete_role(1:string role);
    set<string> get_roles_for_user(1:i64 user);
    set<string> get_implicit_roles_for_user(1:i64 user);
    set<i64> get_users_for_role(1:string role);
    set<i64> get_implicit_users_for_role(1:string role);
    void add_roles_for_user(1:i64 user, 2:set<string> roles);
    void delete_roles_for_user(1:i64 user, 2:set<string> roles);
    set<Permission> get_permissions_for_user(1:i64 user);
    set<Permission> get_implicit_permissions_for_user(1:i64 user);
    void add_permissions_for_user(1:i64 user, 2:set<Permission> permissions);
    void delete_permissions_for_user(1:i64 user, 2:set<Permission> permissions);
    set<Permission> get_permissions_for_role(1:string role);
    set<Permission> get_implicit_permissions_for_role(1:string role);
    void add_permissions_for_role(1:string role, 2:set<Permission> permissions);
    void delete_permissions_for_role(1:string role, 2:set<Permission> permissions);  
}

service Health {
  map<string,string> check();
}
