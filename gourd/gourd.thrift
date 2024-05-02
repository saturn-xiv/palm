namespace cpp gourd.v1
namespace java com.github.saturn_xiv.palm.plugins.gourd.v1

struct Resource{
    1:string type,
    2:optional i64 id,
}

struct Permission{
    1:string operation,
    2:Resource resource,
}


service Policy {
    void can(1:string user, 2:string role);
}

service Health {
  map<string,string> check();
}
