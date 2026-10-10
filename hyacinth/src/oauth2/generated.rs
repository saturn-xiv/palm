#[path="oauth2.u.pb.rs"]
#[allow(nonstandard_style)]
pub mod internal_do_not_use_oauth2;

#[allow(unused_imports, nonstandard_style)]
pub use internal_do_not_use_oauth2::*;
pub mod __unstable {
pub static OAUTH2_DESCRIPTOR_INFO: ::protobuf::__internal::runtime::__unstable::DescriptorInfo = ::protobuf::__internal::runtime::__unstable::DescriptorInfo {
  descriptor: b"\n\x0coauth2.proto\x12\x0epalm.oauth2.v1\">\n\x16GoogleSignInUrlRequest\x12\x14\n\x0credirect_uri\x18\x01 \x01(\t\x12\x0e\n\x06scopes\x18\x02 \x03(\t\"C\n\x17GoogleSignInUrlResponse\x12\x19\n\x11\x61uthorization_url\x18\x01 \x01(\t\x12\r\n\x05state\x18\x02 \x01(\t\"X\n\x17GoogleFetchTokenRequest\x12\x1e\n\x16\x61uthorization_response\x18\x01 \x01(\t\x12\r\n\x05state\x18\x02 \x01(\t\x12\x0e\n\x06scopes\x18\x03 \x03(\t\"\xb2\x01\n\x18GoogleFetchTokenResponse\x12I\n\x0b\x63redentials\x18\x01 \x01(\x0b\x32\x34.palm.oauth2.v1.GoogleFetchTokenResponse.Credentials\x1aK\n\x0b\x43redentials\x12\r\n\x05token\x18\x01 \x01(\t\x12\x15\n\rrefresh_token\x18\x02 \x01(\t\x12\x16\n\x0egranted_scopes\x18\x03 \x03(\t\"\x16\n\x14GoogleRevokeResponse\"\xe6\x02\n\x16GoogleUserInfoResponse\x12\x0b\n\x03sub\x18\x01 \x01(\t\x12\r\n\x05\x65mail\x18\x02 \x01(\t\x12\x16\n\x0everified_email\x18\x03 \x01(\x08\x12\x11\n\x04name\x18\x04 \x01(\tH\x00\x88\x01\x01\x12\x17\n\ngiven_name\x18\x05 \x01(\tH\x01\x88\x01\x01\x12\x18\n\x0b\x66\x61mily_name\x18\x06 \x01(\tH\x02\x88\x01\x01\x12\x14\n\x07picture\x18\x07 \x01(\tH\x03\x88\x01\x01\x12\x13\n\x06locale\x18\x08 \x01(\tH\x04\x88\x01\x01\x12\x1a\n\rhosted_domain\x18\t \x01(\tH\x05\x88\x01\x01\x12\x13\n\x06gender\x18\n \x01(\tH\x06\x88\x01\x01\x12\x11\n\x04link\x18\x0b \x01(\tH\x07\x88\x01\x01\x42\x07\n\x05_nameB\r\n\x0b_given_nameB\x0e\n\x0c_family_nameB\n\n\x08_pictureB\t\n\x07_localeB\x10\n\x0e_hosted_domainB\t\n\x07_genderB\x07\n\x05_link2\x9f\x03\n\x06Google\x12^\n\tSignInUrl\x12&.palm.oauth2.v1.GoogleSignInUrlRequest\x1a\'.palm.oauth2.v1.GoogleSignInUrlResponse\"\x00\x12\x61\n\nFetchToken\x12\'.palm.oauth2.v1.GoogleFetchTokenRequest\x1a(.palm.oauth2.v1.GoogleFetchTokenResponse\"\x00\x12j\n\x08UserInfo\x12\x34.palm.oauth2.v1.GoogleFetchTokenResponse.Credentials\x1a&.palm.oauth2.v1.GoogleUserInfoResponse\"\x00\x12\x66\n\x06Revoke\x12\x34.palm.oauth2.v1.GoogleFetchTokenResponse.Credentials\x1a$.palm.oauth2.v1.GoogleRevokeResponse\"\x00\x42_\n,com.github.saturn_xiv.palm.plugins.oauth2.v1B\x0bOauth2ProtoP\x01Z\x05./;v2\xaa\x02\x18Palm.Plugins.Oauth2.Grpcb\x06proto3",
  deps: &[
  ],
};
}
