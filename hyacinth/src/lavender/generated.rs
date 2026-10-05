#[path="lavender.u.pb.rs"]
#[allow(nonstandard_style)]
pub mod internal_do_not_use_lavender;

#[allow(unused_imports, nonstandard_style)]
pub use internal_do_not_use_lavender::*;
pub mod __unstable {
pub static LAVENDER_DESCRIPTOR_INFO: ::protobuf::__internal::runtime::__unstable::DescriptorInfo = ::protobuf::__internal::runtime::__unstable::DescriptorInfo {
  descriptor: b"\n\x0elavender.proto\x12\x10palm.lavender.v1\"\x07\n\x05\x45mpty\"+\n\tTimestamp\x12\x0f\n\x07seconds\x18\x01 \x01(\x03\x12\r\n\x05nanos\x18\x02 \x01(\x05\"H\n\x04Http\x12\x13\n\x0bstatus_code\x18\x01 \x01(\r\x12\x14\n\x0c\x63ontent_type\x18\x02 \x01(\t\x12\x15\n\rresponse_body\x18\x03 \x01(\t\"\x1d\n\nPostgreSql\x12\x0f\n\x07version\x18\x01 \x01(\t\"\x18\n\x05MySql\x12\x0f\n\x07version\x18\x01 \x01(\t\"7\n\x05Redis\x12\x0c\n\x04info\x18\x01 \x01(\t\x12\x14\n\x07\x63luster\x18\x02 \x01(\tH\x00\x88\x01\x01\x42\n\n\x08_cluster\"w\n\x04Snmp\x12*\n\x05items\x18\x01 \x03(\x0b\x32\x1b.palm.lavender.v1.Snmp.Item\x1a\x43\n\x04Item\x12\x0b\n\x03oid\x18\x01 \x01(\t\x12\x0b\n\x01i\x18\x0b \x01(\x03H\x00\x12\x0b\n\x01s\x18\x0c \x01(\tH\x00\x12\x0b\n\x01\x64\x18\r \x01(\x01H\x00\x42\x07\n\x05value\"\xe0\x02\n\rReportRequest\x12\x33\n\x05items\x18\x01 \x03(\x0b\x32$.palm.lavender.v1.ReportRequest.Item\x1a\x99\x02\n\x04Item\x12.\n\ttimestamp\x18\x01 \x01(\x0b\x32\x1b.palm.lavender.v1.Timestamp\x12&\n\x04http\x18\x0b \x01(\x0b\x32\x16.palm.lavender.v1.HttpH\x00\x12\x32\n\npostgresql\x18\x0c \x01(\x0b\x32\x1c.palm.lavender.v1.PostgreSqlH\x00\x12(\n\x05mysql\x18\r \x01(\x0b\x32\x17.palm.lavender.v1.MySqlH\x00\x12(\n\x05redis\x18\x0e \x01(\x0b\x32\x17.palm.lavender.v1.RedisH\x00\x12&\n\x04snmp\x18\x0f \x01(\x0b\x32\x16.palm.lavender.v1.SnmpH\x00\x42\t\n\x07payload2O\n\x07Storage\x12\x44\n\x06Report\x12\x1f.palm.lavender.v1.ReportRequest\x1a\x17.palm.lavender.v1.Empty\"\x00\x42\x65\n.com.github.saturn_xiv.palm.plugins.lavender.v1B\rLavenderProtoP\x01Z\x05./;v2\xaa\x02\x1aPalm.Plugins.Lavender.Grpcb\x06proto3",
  deps: &[
  ],
};
}
