// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.1
// 	protoc        v3.21.12
// source: s3.proto

package v2

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	durationpb "google.golang.org/protobuf/types/known/durationpb"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Bucket struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Namespace      string `protobuf:"bytes,1,opt,name=namespace,proto3" json:"namespace,omitempty"`
	Name           string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Public         bool   `protobuf:"varint,3,opt,name=public,proto3" json:"public,omitempty"`
	ExpirationDays uint32 `protobuf:"varint,4,opt,name=expiration_days,json=expirationDays,proto3" json:"expiration_days,omitempty"`
}

func (x *Bucket) Reset() {
	*x = Bucket{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Bucket) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Bucket) ProtoMessage() {}

func (x *Bucket) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Bucket.ProtoReflect.Descriptor instead.
func (*Bucket) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{0}
}

func (x *Bucket) GetNamespace() string {
	if x != nil {
		return x.Namespace
	}
	return ""
}

func (x *Bucket) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Bucket) GetPublic() bool {
	if x != nil {
		return x.Public
	}
	return false
}

func (x *Bucket) GetExpirationDays() uint32 {
	if x != nil {
		return x.ExpirationDays
	}
	return 0
}

type CreateBucketRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name           string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	Public         bool   `protobuf:"varint,2,opt,name=public,proto3" json:"public,omitempty"`
	ExpirationDays uint32 `protobuf:"varint,9,opt,name=expiration_days,json=expirationDays,proto3" json:"expiration_days,omitempty"`
}

func (x *CreateBucketRequest) Reset() {
	*x = CreateBucketRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CreateBucketRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CreateBucketRequest) ProtoMessage() {}

func (x *CreateBucketRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CreateBucketRequest.ProtoReflect.Descriptor instead.
func (*CreateBucketRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{1}
}

func (x *CreateBucketRequest) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *CreateBucketRequest) GetPublic() bool {
	if x != nil {
		return x.Public
	}
	return false
}

func (x *CreateBucketRequest) GetExpirationDays() uint32 {
	if x != nil {
		return x.ExpirationDays
	}
	return 0
}

type CreateBucketResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *CreateBucketResponse) Reset() {
	*x = CreateBucketResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CreateBucketResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CreateBucketResponse) ProtoMessage() {}

func (x *CreateBucketResponse) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CreateBucketResponse.ProtoReflect.Descriptor instead.
func (*CreateBucketResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{2}
}

func (x *CreateBucketResponse) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

type UploadRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket string               `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Title  string               `protobuf:"bytes,2,opt,name=title,proto3" json:"title,omitempty"`
	Ttl    *durationpb.Duration `protobuf:"bytes,9,opt,name=ttl,proto3" json:"ttl,omitempty"`
}

func (x *UploadRequest) Reset() {
	*x = UploadRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UploadRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UploadRequest) ProtoMessage() {}

func (x *UploadRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UploadRequest.ProtoReflect.Descriptor instead.
func (*UploadRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{3}
}

func (x *UploadRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *UploadRequest) GetTitle() string {
	if x != nil {
		return x.Title
	}
	return ""
}

func (x *UploadRequest) GetTtl() *durationpb.Duration {
	if x != nil {
		return x.Ttl
	}
	return nil
}

type UploadResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Object string `protobuf:"bytes,1,opt,name=object,proto3" json:"object,omitempty"`
	Url    string `protobuf:"bytes,9,opt,name=url,proto3" json:"url,omitempty"`
}

func (x *UploadResponse) Reset() {
	*x = UploadResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UploadResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UploadResponse) ProtoMessage() {}

func (x *UploadResponse) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UploadResponse.ProtoReflect.Descriptor instead.
func (*UploadResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{4}
}

func (x *UploadResponse) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

func (x *UploadResponse) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

type UrlResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Url string `protobuf:"bytes,1,opt,name=url,proto3" json:"url,omitempty"`
}

func (x *UrlResponse) Reset() {
	*x = UrlResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UrlResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UrlResponse) ProtoMessage() {}

func (x *UrlResponse) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UrlResponse.ProtoReflect.Descriptor instead.
func (*UrlResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{5}
}

func (x *UrlResponse) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

type PresignedUrlRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket      string               `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object      string               `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
	Title       string               `protobuf:"bytes,3,opt,name=title,proto3" json:"title,omitempty"`
	ContentType *string              `protobuf:"bytes,4,opt,name=content_type,json=contentType,proto3,oneof" json:"content_type,omitempty"`
	Ttl         *durationpb.Duration `protobuf:"bytes,9,opt,name=ttl,proto3" json:"ttl,omitempty"`
}

func (x *PresignedUrlRequest) Reset() {
	*x = PresignedUrlRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PresignedUrlRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PresignedUrlRequest) ProtoMessage() {}

func (x *PresignedUrlRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PresignedUrlRequest.ProtoReflect.Descriptor instead.
func (*PresignedUrlRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{6}
}

func (x *PresignedUrlRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *PresignedUrlRequest) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

func (x *PresignedUrlRequest) GetTitle() string {
	if x != nil {
		return x.Title
	}
	return ""
}

func (x *PresignedUrlRequest) GetContentType() string {
	if x != nil && x.ContentType != nil {
		return *x.ContentType
	}
	return ""
}

func (x *PresignedUrlRequest) GetTtl() *durationpb.Duration {
	if x != nil {
		return x.Ttl
	}
	return nil
}

type PermanentUrlRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket string `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object string `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
}

func (x *PermanentUrlRequest) Reset() {
	*x = PermanentUrlRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PermanentUrlRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PermanentUrlRequest) ProtoMessage() {}

func (x *PermanentUrlRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use PermanentUrlRequest.ProtoReflect.Descriptor instead.
func (*PermanentUrlRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{7}
}

func (x *PermanentUrlRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *PermanentUrlRequest) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

var File_s3_proto protoreflect.FileDescriptor

var file_s3_proto_rawDesc = []byte{
	0x0a, 0x08, 0x73, 0x33, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7b, 0x0a, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
	0x12, 0x1c, 0x0a, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x12, 0x12,
	0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x08, 0x52, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x27, 0x0a, 0x0f, 0x65, 0x78,
	0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x79, 0x73, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x0d, 0x52, 0x0e, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44,
	0x61, 0x79, 0x73, 0x22, 0x6a, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63,
	0x6b, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16,
	0x0a, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06,
	0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x27, 0x0a, 0x0f, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x79, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52,
	0x0e, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61, 0x79, 0x73, 0x22,
	0x2a, 0x0a, 0x14, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x6a, 0x0a, 0x0d, 0x55,
	0x70, 0x6c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06,
	0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x62, 0x75,
	0x63, 0x6b, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x2b, 0x0a, 0x03, 0x74, 0x74,
	0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x22, 0x3a, 0x0a, 0x0e, 0x55, 0x70, 0x6c, 0x6f, 0x61,
	0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x62, 0x6a,
	0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
	0x75, 0x72, 0x6c, 0x22, 0x1f, 0x0a, 0x0b, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x03, 0x75, 0x72, 0x6c, 0x22, 0xc1, 0x01, 0x0a, 0x13, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e,
	0x65, 0x64, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06,
	0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x62, 0x75,
	0x63, 0x6b, 0x65, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x14, 0x0a, 0x05,
	0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74,
	0x6c, 0x65, 0x12, 0x26, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79,
	0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74,
	0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x88, 0x01, 0x01, 0x12, 0x2b, 0x0a, 0x03, 0x74, 0x74,
	0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
	0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x22, 0x45, 0x0a, 0x13, 0x50, 0x65, 0x72, 0x6d,
	0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x32,
	0xb4, 0x02, 0x0a, 0x02, 0x53, 0x33, 0x12, 0x53, 0x0a, 0x0c, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
	0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1f, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33,
	0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73,
	0x33, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65,
	0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x41, 0x0a, 0x06, 0x55,
	0x70, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x19, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e,
	0x76, 0x31, 0x2e, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x1a, 0x1a, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x70,
	0x6c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x4a,
	0x0a, 0x0c, 0x50, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x12, 0x1f,
	0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x65, 0x72, 0x6d,
	0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
	0x17, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x72, 0x6c,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x4a, 0x0a, 0x0c, 0x50, 0x72,
	0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x55, 0x72, 0x6c, 0x12, 0x1f, 0x2e, 0x70, 0x61, 0x6c,
	0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65,
	0x64, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x17, 0x2e, 0x70, 0x61,
	0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0x57, 0x0a, 0x28, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x78, 0x69, 0x76, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x73, 0x2e, 0x73, 0x33, 0x2e,
	0x76, 0x31, 0x50, 0x01, 0x5a, 0x29, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
	0x2f, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x2d, 0x78, 0x69, 0x76, 0x2f, 0x70, 0x61, 0x6c, 0x6d,
	0x2f, 0x73, 0x33, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x2f, 0x76, 0x32, 0x62,
	0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_s3_proto_rawDescOnce sync.Once
	file_s3_proto_rawDescData = file_s3_proto_rawDesc
)

func file_s3_proto_rawDescGZIP() []byte {
	file_s3_proto_rawDescOnce.Do(func() {
		file_s3_proto_rawDescData = protoimpl.X.CompressGZIP(file_s3_proto_rawDescData)
	})
	return file_s3_proto_rawDescData
}

var file_s3_proto_msgTypes = make([]protoimpl.MessageInfo, 8)
var file_s3_proto_goTypes = []interface{}{
	(*Bucket)(nil),               // 0: palm.s3.v1.Bucket
	(*CreateBucketRequest)(nil),  // 1: palm.s3.v1.CreateBucketRequest
	(*CreateBucketResponse)(nil), // 2: palm.s3.v1.CreateBucketResponse
	(*UploadRequest)(nil),        // 3: palm.s3.v1.UploadRequest
	(*UploadResponse)(nil),       // 4: palm.s3.v1.UploadResponse
	(*UrlResponse)(nil),          // 5: palm.s3.v1.UrlResponse
	(*PresignedUrlRequest)(nil),  // 6: palm.s3.v1.PresignedUrlRequest
	(*PermanentUrlRequest)(nil),  // 7: palm.s3.v1.PermanentUrlRequest
	(*durationpb.Duration)(nil),  // 8: google.protobuf.Duration
}
var file_s3_proto_depIdxs = []int32{
	8, // 0: palm.s3.v1.UploadRequest.ttl:type_name -> google.protobuf.Duration
	8, // 1: palm.s3.v1.PresignedUrlRequest.ttl:type_name -> google.protobuf.Duration
	1, // 2: palm.s3.v1.S3.CreateBucket:input_type -> palm.s3.v1.CreateBucketRequest
	3, // 3: palm.s3.v1.S3.Upload:input_type -> palm.s3.v1.UploadRequest
	7, // 4: palm.s3.v1.S3.PermanentUrl:input_type -> palm.s3.v1.PermanentUrlRequest
	6, // 5: palm.s3.v1.S3.PresignedUrl:input_type -> palm.s3.v1.PresignedUrlRequest
	2, // 6: palm.s3.v1.S3.CreateBucket:output_type -> palm.s3.v1.CreateBucketResponse
	4, // 7: palm.s3.v1.S3.Upload:output_type -> palm.s3.v1.UploadResponse
	5, // 8: palm.s3.v1.S3.PermanentUrl:output_type -> palm.s3.v1.UrlResponse
	5, // 9: palm.s3.v1.S3.PresignedUrl:output_type -> palm.s3.v1.UrlResponse
	6, // [6:10] is the sub-list for method output_type
	2, // [2:6] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_s3_proto_init() }
func file_s3_proto_init() {
	if File_s3_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_s3_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Bucket); i {
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
		file_s3_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CreateBucketRequest); i {
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
		file_s3_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CreateBucketResponse); i {
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
		file_s3_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UploadRequest); i {
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
		file_s3_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UploadResponse); i {
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
		file_s3_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UrlResponse); i {
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
		file_s3_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PresignedUrlRequest); i {
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
		file_s3_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*PermanentUrlRequest); i {
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
	file_s3_proto_msgTypes[6].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_s3_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   8,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_s3_proto_goTypes,
		DependencyIndexes: file_s3_proto_depIdxs,
		MessageInfos:      file_s3_proto_msgTypes,
	}.Build()
	File_s3_proto = out.File
	file_s3_proto_rawDesc = nil
	file_s3_proto_goTypes = nil
	file_s3_proto_depIdxs = nil
}
