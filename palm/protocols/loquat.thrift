namespace cpp loquat.v1
namespace java com.github.saturn_xiv.palm.plugins.loquat.v1

service Health {
  void check();
}

struct Resource {
    1:string type,
    2:optional i32 id,
}
struct Permission {
    1:string operation,
    2:Resource resource,
}

service Auth {
    void sign_up(1:string user, 2:string password);
    void sign_in(1:string user, 2:string password, 3:i64 ttl);
    void change_password(1:string user, 2:string current_password, 3:string new_password);
    void reset_password(1:string token, 2:string user, 3:string password);
    
    bool has(1:string token, 2:string role);
    bool can(1:string token, 2:string operation, 3:string resource);
    set<string> roles(1:string token);
    set<Permission> permissions(1:string token);
}

service Policy {
    void add_roles_for_user(1:string token, 2:string user, 3:set<string> roles);
    set<string> get_roles_for_user(1:string token, 2:string user);
    set<string> get_users_for_role(1:string token, 2:string role);    
    void delete_roles_for_user(1:string token, 2:string user, 3:set<string> roles);
    void delete_users_for_role(1:string token, 2:string role, 3:set<string> users);
    void delete_role(1:string token, 2:string role);
    void delete_user(1:string token, 2:string user);
    void add_permissions_for_user(1:string token, 2:string user, 3:set<Permission> permissions);
    void add_permissions_for_role(1:string token, 2:string role, 3:set<Permission> permissions);
    void delete_permissions_for_user(1:string token, 2:string user, 3:set<Permission> permissions);
    void delete_permissions_for_role(1:string token, 2:string role, 3:set<Permission> permissions);
    set<Permission> get_permissions_for_user(1:string token, 2:string user);
    set<Permission> get_permissions_for_role(1:string token, 2:string role);
}

service Token{
    string sign(1:string token, 2:string issuer, 3:string subject, 4:string audience, 5:i64 ttl);
    string verify(1:string token, 2:string issuer, 3:string audience);
}
