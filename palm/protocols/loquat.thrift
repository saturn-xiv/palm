namespace cpp loquat.v1
namespace java com.github.saturn_xiv.palm.plugins.loquat.v1

struct JwtVerfifyResponse{
    1:optional string jwt_id,
    2:optional string key_id,
    8:required string subject,
    9:optional binary payload,
}

struct JwtSignRequest{
    1:optional string jwt_id,
    2:optional string key_id,
    
    11:required string issuer,
    12:required string subject,
    13:required set<string> audiences,
    
    21:required i64 issued_at,
    22:required i64 not_before,
    23:required i64 expired_at,
    
    99:optional binary payload,
}

service Jwt {
    string sign(1:JwtSignRequest request);
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
  map<string,string> check();
}
