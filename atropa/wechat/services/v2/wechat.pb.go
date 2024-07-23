// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.1
// 	protoc        v3.21.12
// source: wechat.proto

package v2

import (
	v2 "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Oauth2QrConnectUrlRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Subject *string                                    `protobuf:"bytes,1,opt,name=subject,proto3,oneof" json:"subject,omitempty"`
	Lang    v2.WechatOauth2UserIndexResponse_Item_Lang `protobuf:"varint,9,opt,name=lang,proto3,enum=palm.balsam.v1.WechatOauth2UserIndexResponse_Item_Lang" json:"lang,omitempty"`
}

func (x *Oauth2QrConnectUrlRequest) Reset() {
	*x = Oauth2QrConnectUrlRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Oauth2QrConnectUrlRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Oauth2QrConnectUrlRequest) ProtoMessage() {}

func (x *Oauth2QrConnectUrlRequest) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Oauth2QrConnectUrlRequest.ProtoReflect.Descriptor instead.
func (*Oauth2QrConnectUrlRequest) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{0}
}

func (x *Oauth2QrConnectUrlRequest) GetSubject() string {
	if x != nil && x.Subject != nil {
		return *x.Subject
	}
	return ""
}

func (x *Oauth2QrConnectUrlRequest) GetLang() v2.WechatOauth2UserIndexResponse_Item_Lang {
	if x != nil {
		return x.Lang
	}
	return v2.WechatOauth2UserIndexResponse_Item_Lang(0)
}

type Oauth2QrConnectUrlResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Url string `protobuf:"bytes,1,opt,name=url,proto3" json:"url,omitempty"`
}

func (x *Oauth2QrConnectUrlResponse) Reset() {
	*x = Oauth2QrConnectUrlResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Oauth2QrConnectUrlResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Oauth2QrConnectUrlResponse) ProtoMessage() {}

func (x *Oauth2QrConnectUrlResponse) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Oauth2QrConnectUrlResponse.ProtoReflect.Descriptor instead.
func (*Oauth2QrConnectUrlResponse) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{1}
}

func (x *Oauth2QrConnectUrlResponse) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

type Oauth2SignInRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Code  string                                     `protobuf:"bytes,1,opt,name=code,proto3" json:"code,omitempty"`
	State string                                     `protobuf:"bytes,2,opt,name=state,proto3" json:"state,omitempty"`
	Lang  v2.WechatOauth2UserIndexResponse_Item_Lang `protobuf:"varint,9,opt,name=lang,proto3,enum=palm.balsam.v1.WechatOauth2UserIndexResponse_Item_Lang" json:"lang,omitempty"`
}

func (x *Oauth2SignInRequest) Reset() {
	*x = Oauth2SignInRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Oauth2SignInRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Oauth2SignInRequest) ProtoMessage() {}

func (x *Oauth2SignInRequest) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Oauth2SignInRequest.ProtoReflect.Descriptor instead.
func (*Oauth2SignInRequest) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{2}
}

func (x *Oauth2SignInRequest) GetCode() string {
	if x != nil {
		return x.Code
	}
	return ""
}

func (x *Oauth2SignInRequest) GetState() string {
	if x != nil {
		return x.State
	}
	return ""
}

func (x *Oauth2SignInRequest) GetLang() v2.WechatOauth2UserIndexResponse_Item_Lang {
	if x != nil {
		return x.Lang
	}
	return v2.WechatOauth2UserIndexResponse_Item_Lang(0)
}

type Oauth2SignInResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UserInfo *Oauth2SignInResponse_UserInfo `protobuf:"bytes,1,opt,name=user_info,json=userInfo,proto3" json:"user_info,omitempty"`
	Token    []byte                         `protobuf:"bytes,2,opt,name=token,proto3" json:"token,omitempty"`
	Subject  *string                        `protobuf:"bytes,9,opt,name=subject,proto3,oneof" json:"subject,omitempty"`
}

func (x *Oauth2SignInResponse) Reset() {
	*x = Oauth2SignInResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Oauth2SignInResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Oauth2SignInResponse) ProtoMessage() {}

func (x *Oauth2SignInResponse) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Oauth2SignInResponse.ProtoReflect.Descriptor instead.
func (*Oauth2SignInResponse) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{3}
}

func (x *Oauth2SignInResponse) GetUserInfo() *Oauth2SignInResponse_UserInfo {
	if x != nil {
		return x.UserInfo
	}
	return nil
}

func (x *Oauth2SignInResponse) GetToken() []byte {
	if x != nil {
		return x.Token
	}
	return nil
}

func (x *Oauth2SignInResponse) GetSubject() string {
	if x != nil && x.Subject != nil {
		return *x.Subject
	}
	return ""
}

type MiniProgramCode2SessionRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Code string `protobuf:"bytes,1,opt,name=code,proto3" json:"code,omitempty"`
}

func (x *MiniProgramCode2SessionRequest) Reset() {
	*x = MiniProgramCode2SessionRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MiniProgramCode2SessionRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MiniProgramCode2SessionRequest) ProtoMessage() {}

func (x *MiniProgramCode2SessionRequest) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MiniProgramCode2SessionRequest.ProtoReflect.Descriptor instead.
func (*MiniProgramCode2SessionRequest) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{4}
}

func (x *MiniProgramCode2SessionRequest) GetCode() string {
	if x != nil {
		return x.Code
	}
	return ""
}

type MiniProgramCode2SessionResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UnionId *string `protobuf:"bytes,1,opt,name=union_id,json=unionId,proto3,oneof" json:"union_id,omitempty"`
	OpenId  string  `protobuf:"bytes,2,opt,name=open_id,json=openId,proto3" json:"open_id,omitempty"`
}

func (x *MiniProgramCode2SessionResponse) Reset() {
	*x = MiniProgramCode2SessionResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MiniProgramCode2SessionResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MiniProgramCode2SessionResponse) ProtoMessage() {}

func (x *MiniProgramCode2SessionResponse) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MiniProgramCode2SessionResponse.ProtoReflect.Descriptor instead.
func (*MiniProgramCode2SessionResponse) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{5}
}

func (x *MiniProgramCode2SessionResponse) GetUnionId() string {
	if x != nil && x.UnionId != nil {
		return *x.UnionId
	}
	return ""
}

func (x *MiniProgramCode2SessionResponse) GetOpenId() string {
	if x != nil {
		return x.OpenId
	}
	return ""
}

type Oauth2SignInResponse_UserInfo struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	OpenId     string   `protobuf:"bytes,1,opt,name=open_id,json=openId,proto3" json:"open_id,omitempty"`
	Nickname   string   `protobuf:"bytes,2,opt,name=nickname,proto3" json:"nickname,omitempty"`
	Sex        uint32   `protobuf:"varint,3,opt,name=sex,proto3" json:"sex,omitempty"`
	Province   string   `protobuf:"bytes,4,opt,name=province,proto3" json:"province,omitempty"`
	City       string   `protobuf:"bytes,5,opt,name=city,proto3" json:"city,omitempty"`
	Country    string   `protobuf:"bytes,6,opt,name=country,proto3" json:"country,omitempty"`
	HeadImgUrl *string  `protobuf:"bytes,7,opt,name=head_img_url,json=headImgUrl,proto3,oneof" json:"head_img_url,omitempty"`
	Privilege  []string `protobuf:"bytes,8,rep,name=privilege,proto3" json:"privilege,omitempty"`
	UnionId    string   `protobuf:"bytes,9,opt,name=union_id,json=unionId,proto3" json:"union_id,omitempty"`
}

func (x *Oauth2SignInResponse_UserInfo) Reset() {
	*x = Oauth2SignInResponse_UserInfo{}
	if protoimpl.UnsafeEnabled {
		mi := &file_wechat_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Oauth2SignInResponse_UserInfo) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Oauth2SignInResponse_UserInfo) ProtoMessage() {}

func (x *Oauth2SignInResponse_UserInfo) ProtoReflect() protoreflect.Message {
	mi := &file_wechat_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Oauth2SignInResponse_UserInfo.ProtoReflect.Descriptor instead.
func (*Oauth2SignInResponse_UserInfo) Descriptor() ([]byte, []int) {
	return file_wechat_proto_rawDescGZIP(), []int{3, 0}
}

func (x *Oauth2SignInResponse_UserInfo) GetOpenId() string {
	if x != nil {
		return x.OpenId
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetNickname() string {
	if x != nil {
		return x.Nickname
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetSex() uint32 {
	if x != nil {
		return x.Sex
	}
	return 0
}

func (x *Oauth2SignInResponse_UserInfo) GetProvince() string {
	if x != nil {
		return x.Province
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetCity() string {
	if x != nil {
		return x.City
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetCountry() string {
	if x != nil {
		return x.Country
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetHeadImgUrl() string {
	if x != nil && x.HeadImgUrl != nil {
		return *x.HeadImgUrl
	}
	return ""
}

func (x *Oauth2SignInResponse_UserInfo) GetPrivilege() []string {
	if x != nil {
		return x.Privilege
	}
	return nil
}

func (x *Oauth2SignInResponse_UserInfo) GetUnionId() string {
	if x != nil {
		return x.UnionId
	}
	return ""
}

var File_wechat_proto protoreflect.FileDescriptor

var file_wechat_proto_rawDesc = []byte{
	0x0a, 0x0c, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76, 0x31, 0x1a, 0x0c,
	0x62, 0x61, 0x6c, 0x73, 0x61, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x93, 0x01, 0x0a,
	0x19, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x51, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
	0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x07, 0x73, 0x75,
	0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x07, 0x73,
	0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x88, 0x01, 0x01, 0x12, 0x4b, 0x0a, 0x04, 0x6c, 0x61, 0x6e,
	0x67, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x37, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x62,
	0x61, 0x6c, 0x73, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x57, 0x65, 0x63, 0x68, 0x61, 0x74, 0x4f,
	0x61, 0x75, 0x74, 0x68, 0x32, 0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x4c, 0x61, 0x6e, 0x67,
	0x52, 0x04, 0x6c, 0x61, 0x6e, 0x67, 0x42, 0x0a, 0x0a, 0x08, 0x5f, 0x73, 0x75, 0x62, 0x6a, 0x65,
	0x63, 0x74, 0x22, 0x2e, 0x0a, 0x1a, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x51, 0x72, 0x43, 0x6f,
	0x6e, 0x6e, 0x65, 0x63, 0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75,
	0x72, 0x6c, 0x22, 0x8c, 0x01, 0x0a, 0x13, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x53, 0x69, 0x67,
	0x6e, 0x49, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f,
	0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x14,
	0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x73,
	0x74, 0x61, 0x74, 0x65, 0x12, 0x4b, 0x0a, 0x04, 0x6c, 0x61, 0x6e, 0x67, 0x18, 0x09, 0x20, 0x01,
	0x28, 0x0e, 0x32, 0x37, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x62, 0x61, 0x6c, 0x73, 0x61, 0x6d,
	0x2e, 0x76, 0x31, 0x2e, 0x57, 0x65, 0x63, 0x68, 0x61, 0x74, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32,
	0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x4c, 0x61, 0x6e, 0x67, 0x52, 0x04, 0x6c, 0x61, 0x6e,
	0x67, 0x22, 0xb2, 0x03, 0x0a, 0x14, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x53, 0x69, 0x67, 0x6e,
	0x49, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4a, 0x0a, 0x09, 0x75, 0x73,
	0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4f,
	0x61, 0x75, 0x74, 0x68, 0x32, 0x53, 0x69, 0x67, 0x6e, 0x49, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x08, 0x75, 0x73,
	0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1d, 0x0a, 0x07,
	0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52,
	0x07, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x88, 0x01, 0x01, 0x1a, 0x8c, 0x02, 0x0a, 0x08,
	0x55, 0x73, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x17, 0x0a, 0x07, 0x6f, 0x70, 0x65, 0x6e,
	0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x70, 0x65, 0x6e, 0x49,
	0x64, 0x12, 0x1a, 0x0a, 0x08, 0x6e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x08, 0x6e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10, 0x0a,
	0x03, 0x73, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x03, 0x73, 0x65, 0x78, 0x12,
	0x1a, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x6e, 0x63, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x6e, 0x63, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x63,
	0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x63, 0x69, 0x74, 0x79, 0x12,
	0x18, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x25, 0x0a, 0x0c, 0x68, 0x65, 0x61,
	0x64, 0x5f, 0x69, 0x6d, 0x67, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x48,
	0x00, 0x52, 0x0a, 0x68, 0x65, 0x61, 0x64, 0x49, 0x6d, 0x67, 0x55, 0x72, 0x6c, 0x88, 0x01, 0x01,
	0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x69, 0x76, 0x69, 0x6c, 0x65, 0x67, 0x65, 0x18, 0x08, 0x20,
	0x03, 0x28, 0x09, 0x52, 0x09, 0x70, 0x72, 0x69, 0x76, 0x69, 0x6c, 0x65, 0x67, 0x65, 0x12, 0x19,
	0x0a, 0x08, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x07, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x68, 0x65,
	0x61, 0x64, 0x5f, 0x69, 0x6d, 0x67, 0x5f, 0x75, 0x72, 0x6c, 0x42, 0x0a, 0x0a, 0x08, 0x5f, 0x73,
	0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x22, 0x34, 0x0a, 0x1e, 0x4d, 0x69, 0x6e, 0x69, 0x50, 0x72,
	0x6f, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x64, 0x65, 0x32, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
	0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x22, 0x67, 0x0a, 0x1f,
	0x4d, 0x69, 0x6e, 0x69, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x64, 0x65, 0x32,
	0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
	0x1e, 0x0a, 0x08, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x48, 0x00, 0x52, 0x07, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x88, 0x01, 0x01, 0x12,
	0x17, 0x0a, 0x07, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x06, 0x6f, 0x70, 0x65, 0x6e, 0x49, 0x64, 0x42, 0x0b, 0x0a, 0x09, 0x5f, 0x75, 0x6e, 0x69,
	0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x32, 0xc8, 0x01, 0x0a, 0x06, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32,
	0x12, 0x67, 0x0a, 0x0c, 0x51, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x55, 0x72, 0x6c,
	0x12, 0x29, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76,
	0x31, 0x2e, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x51, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
	0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x70, 0x61,
	0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x61, 0x75,
	0x74, 0x68, 0x32, 0x51, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x55, 0x72, 0x6c, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x55, 0x0a, 0x06, 0x53, 0x69, 0x67,
	0x6e, 0x49, 0x6e, 0x12, 0x23, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61,
	0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32, 0x53, 0x69, 0x67, 0x6e, 0x49,
	0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x24, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e,
	0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x61, 0x75, 0x74, 0x68, 0x32,
	0x53, 0x69, 0x67, 0x6e, 0x49, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00,
	0x32, 0x80, 0x01, 0x0a, 0x0b, 0x4d, 0x69, 0x6e, 0x69, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d,
	0x12, 0x71, 0x0a, 0x0c, 0x43, 0x6f, 0x64, 0x65, 0x32, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
	0x12, 0x2e, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76,
	0x31, 0x2e, 0x4d, 0x69, 0x6e, 0x69, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x64,
	0x65, 0x32, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x1a, 0x2f, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76,
	0x31, 0x2e, 0x4d, 0x69, 0x6e, 0x69, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x64,
	0x65, 0x32, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x22, 0x00, 0x32, 0x05, 0x0a, 0x03, 0x50, 0x61, 0x79, 0x42, 0x66, 0x0a, 0x2c, 0x63, 0x6f,
	0x6d, 0x2e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x5f,
	0x78, 0x69, 0x76, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x73,
	0x2e, 0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x76, 0x31, 0x50, 0x01, 0x5a, 0x34, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x2d,
	0x78, 0x69, 0x76, 0x2f, 0x70, 0x61, 0x6c, 0x6d, 0x2f, 0x61, 0x74, 0x72, 0x6f, 0x70, 0x61, 0x2f,
	0x77, 0x65, 0x63, 0x68, 0x61, 0x74, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x2f,
	0x76, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_wechat_proto_rawDescOnce sync.Once
	file_wechat_proto_rawDescData = file_wechat_proto_rawDesc
)

func file_wechat_proto_rawDescGZIP() []byte {
	file_wechat_proto_rawDescOnce.Do(func() {
		file_wechat_proto_rawDescData = protoimpl.X.CompressGZIP(file_wechat_proto_rawDescData)
	})
	return file_wechat_proto_rawDescData
}

var file_wechat_proto_msgTypes = make([]protoimpl.MessageInfo, 7)
var file_wechat_proto_goTypes = []interface{}{
	(*Oauth2QrConnectUrlRequest)(nil),               // 0: palm.wechat.v1.Oauth2QrConnectUrlRequest
	(*Oauth2QrConnectUrlResponse)(nil),              // 1: palm.wechat.v1.Oauth2QrConnectUrlResponse
	(*Oauth2SignInRequest)(nil),                     // 2: palm.wechat.v1.Oauth2SignInRequest
	(*Oauth2SignInResponse)(nil),                    // 3: palm.wechat.v1.Oauth2SignInResponse
	(*MiniProgramCode2SessionRequest)(nil),          // 4: palm.wechat.v1.MiniProgramCode2SessionRequest
	(*MiniProgramCode2SessionResponse)(nil),         // 5: palm.wechat.v1.MiniProgramCode2SessionResponse
	(*Oauth2SignInResponse_UserInfo)(nil),           // 6: palm.wechat.v1.Oauth2SignInResponse.UserInfo
	(v2.WechatOauth2UserIndexResponse_Item_Lang)(0), // 7: palm.balsam.v1.WechatOauth2UserIndexResponse.Item.Lang
}
var file_wechat_proto_depIdxs = []int32{
	7, // 0: palm.wechat.v1.Oauth2QrConnectUrlRequest.lang:type_name -> palm.balsam.v1.WechatOauth2UserIndexResponse.Item.Lang
	7, // 1: palm.wechat.v1.Oauth2SignInRequest.lang:type_name -> palm.balsam.v1.WechatOauth2UserIndexResponse.Item.Lang
	6, // 2: palm.wechat.v1.Oauth2SignInResponse.user_info:type_name -> palm.wechat.v1.Oauth2SignInResponse.UserInfo
	0, // 3: palm.wechat.v1.Oauth2.QrConnectUrl:input_type -> palm.wechat.v1.Oauth2QrConnectUrlRequest
	2, // 4: palm.wechat.v1.Oauth2.SignIn:input_type -> palm.wechat.v1.Oauth2SignInRequest
	4, // 5: palm.wechat.v1.MiniProgram.Code2Session:input_type -> palm.wechat.v1.MiniProgramCode2SessionRequest
	1, // 6: palm.wechat.v1.Oauth2.QrConnectUrl:output_type -> palm.wechat.v1.Oauth2QrConnectUrlResponse
	3, // 7: palm.wechat.v1.Oauth2.SignIn:output_type -> palm.wechat.v1.Oauth2SignInResponse
	5, // 8: palm.wechat.v1.MiniProgram.Code2Session:output_type -> palm.wechat.v1.MiniProgramCode2SessionResponse
	6, // [6:9] is the sub-list for method output_type
	3, // [3:6] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_wechat_proto_init() }
func file_wechat_proto_init() {
	if File_wechat_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_wechat_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Oauth2QrConnectUrlRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Oauth2QrConnectUrlResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Oauth2SignInRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Oauth2SignInResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MiniProgramCode2SessionRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MiniProgramCode2SessionResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_wechat_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Oauth2SignInResponse_UserInfo); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_wechat_proto_msgTypes[0].OneofWrappers = []interface{}{}
	file_wechat_proto_msgTypes[3].OneofWrappers = []interface{}{}
	file_wechat_proto_msgTypes[5].OneofWrappers = []interface{}{}
	file_wechat_proto_msgTypes[6].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_wechat_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   7,
			NumExtensions: 0,
			NumServices:   3,
		},
		GoTypes:           file_wechat_proto_goTypes,
		DependencyIndexes: file_wechat_proto_depIdxs,
		MessageInfos:      file_wechat_proto_msgTypes,
	}.Build()
	File_wechat_proto = out.File
	file_wechat_proto_rawDesc = nil
	file_wechat_proto_goTypes = nil
	file_wechat_proto_depIdxs = nil
}
