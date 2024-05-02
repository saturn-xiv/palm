namespace cpp loquat.v1
namespace java com.github.saturn_xiv.palm.plugins.loquat.v1

struct JwtVerfifyResponse{
    1:string subject,
    2:optional string payload,
}

service Jwt {
    string sign(1:string issuer, 2:string subject, 3:set<string> audience, 4:i64 ttl, 5: optional string payload);
    JwtVerfifyResponse verify(1:string token, 2:string issuer, 3:string audience);
}

service Hmac {
    binary sign(1:binary plain);
    void verify(1:binary code, 2:binary plain);
}

service Aes {
    binary encrypt(1:binary plain);
    binary decrypt(1:binary code);
}

service Health {
  void check();
}
