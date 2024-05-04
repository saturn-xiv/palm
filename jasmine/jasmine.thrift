namespace cpp jasmine.v1
namespace java com.github.saturn_xiv.palm.plugins.jasmine.v1

service S3 {
    void create_bucket(1:string name, 2:bool public, 3:i8 expiration_days);
    string upload_file(1:string bucket, 2:string object, 3:i32 ttl);
    string get_presigned_url(1:string bucket, 2:string object, 3:string title, 4:i32 ttl);
    string get_permanent_url(1:string bucket, 2:string object);
}
