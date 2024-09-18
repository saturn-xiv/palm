// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v5.27.2
// source: s3.proto

package v2

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	durationpb "google.golang.org/protobuf/types/known/durationpb"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
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

	Name           string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	Public         bool   `protobuf:"varint,2,opt,name=public,proto3" json:"public,omitempty"`
	ExpirationDays int32  `protobuf:"varint,3,opt,name=expiration_days,json=expirationDays,proto3" json:"expiration_days,omitempty"`
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

func (x *Bucket) GetExpirationDays() int32 {
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

type ListBucketResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Items []*ListBucketResponse_Item `protobuf:"bytes,1,rep,name=items,proto3" json:"items,omitempty"`
}

func (x *ListBucketResponse) Reset() {
	*x = ListBucketResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListBucketResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListBucketResponse) ProtoMessage() {}

func (x *ListBucketResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use ListBucketResponse.ProtoReflect.Descriptor instead.
func (*ListBucketResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{3}
}

func (x *ListBucketResponse) GetItems() []*ListBucketResponse_Item {
	if x != nil {
		return x.Items
	}
	return nil
}

type PresignedPutObjectRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket string               `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Title  string               `protobuf:"bytes,2,opt,name=title,proto3" json:"title,omitempty"`
	Ttl    *durationpb.Duration `protobuf:"bytes,9,opt,name=ttl,proto3" json:"ttl,omitempty"`
}

func (x *PresignedPutObjectRequest) Reset() {
	*x = PresignedPutObjectRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PresignedPutObjectRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PresignedPutObjectRequest) ProtoMessage() {}

func (x *PresignedPutObjectRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use PresignedPutObjectRequest.ProtoReflect.Descriptor instead.
func (*PresignedPutObjectRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{4}
}

func (x *PresignedPutObjectRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *PresignedPutObjectRequest) GetTitle() string {
	if x != nil {
		return x.Title
	}
	return ""
}

func (x *PresignedPutObjectRequest) GetTtl() *durationpb.Duration {
	if x != nil {
		return x.Ttl
	}
	return nil
}

type PresignedPutObjectResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Object string `protobuf:"bytes,1,opt,name=object,proto3" json:"object,omitempty"`
	Url    string `protobuf:"bytes,9,opt,name=url,proto3" json:"url,omitempty"`
}

func (x *PresignedPutObjectResponse) Reset() {
	*x = PresignedPutObjectResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *PresignedPutObjectResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*PresignedPutObjectResponse) ProtoMessage() {}

func (x *PresignedPutObjectResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use PresignedPutObjectResponse.ProtoReflect.Descriptor instead.
func (*PresignedPutObjectResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{5}
}

func (x *PresignedPutObjectResponse) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

func (x *PresignedPutObjectResponse) GetUrl() string {
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
		mi := &file_s3_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UrlResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UrlResponse) ProtoMessage() {}

func (x *UrlResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use UrlResponse.ProtoReflect.Descriptor instead.
func (*UrlResponse) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{6}
}

func (x *UrlResponse) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

type ObjectPresignedUrlRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket      string               `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object      string               `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
	Title       string               `protobuf:"bytes,3,opt,name=title,proto3" json:"title,omitempty"`
	ContentType *string              `protobuf:"bytes,4,opt,name=content_type,json=contentType,proto3,oneof" json:"content_type,omitempty"`
	Ttl         *durationpb.Duration `protobuf:"bytes,9,opt,name=ttl,proto3" json:"ttl,omitempty"`
}

func (x *ObjectPresignedUrlRequest) Reset() {
	*x = ObjectPresignedUrlRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ObjectPresignedUrlRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ObjectPresignedUrlRequest) ProtoMessage() {}

func (x *ObjectPresignedUrlRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use ObjectPresignedUrlRequest.ProtoReflect.Descriptor instead.
func (*ObjectPresignedUrlRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{7}
}

func (x *ObjectPresignedUrlRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *ObjectPresignedUrlRequest) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

func (x *ObjectPresignedUrlRequest) GetTitle() string {
	if x != nil {
		return x.Title
	}
	return ""
}

func (x *ObjectPresignedUrlRequest) GetContentType() string {
	if x != nil && x.ContentType != nil {
		return *x.ContentType
	}
	return ""
}

func (x *ObjectPresignedUrlRequest) GetTtl() *durationpb.Duration {
	if x != nil {
		return x.Ttl
	}
	return nil
}

type ObjectPermanentUrlRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket      string  `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object      string  `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
	Title       string  `protobuf:"bytes,3,opt,name=title,proto3" json:"title,omitempty"`
	ContentType *string `protobuf:"bytes,4,opt,name=content_type,json=contentType,proto3,oneof" json:"content_type,omitempty"`
}

func (x *ObjectPermanentUrlRequest) Reset() {
	*x = ObjectPermanentUrlRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ObjectPermanentUrlRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ObjectPermanentUrlRequest) ProtoMessage() {}

func (x *ObjectPermanentUrlRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ObjectPermanentUrlRequest.ProtoReflect.Descriptor instead.
func (*ObjectPermanentUrlRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{8}
}

func (x *ObjectPermanentUrlRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *ObjectPermanentUrlRequest) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

func (x *ObjectPermanentUrlRequest) GetTitle() string {
	if x != nil {
		return x.Title
	}
	return ""
}

func (x *ObjectPermanentUrlRequest) GetContentType() string {
	if x != nil && x.ContentType != nil {
		return *x.ContentType
	}
	return ""
}

type RemoveObjectRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket string `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object string `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
}

func (x *RemoveObjectRequest) Reset() {
	*x = RemoveObjectRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[9]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RemoveObjectRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RemoveObjectRequest) ProtoMessage() {}

func (x *RemoveObjectRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[9]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RemoveObjectRequest.ProtoReflect.Descriptor instead.
func (*RemoveObjectRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{9}
}

func (x *RemoveObjectRequest) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *RemoveObjectRequest) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

type RemoveBucketRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *RemoveBucketRequest) Reset() {
	*x = RemoveBucketRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[10]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RemoveBucketRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RemoveBucketRequest) ProtoMessage() {}

func (x *RemoveBucketRequest) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[10]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RemoveBucketRequest.ProtoReflect.Descriptor instead.
func (*RemoveBucketRequest) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{10}
}

func (x *RemoveBucketRequest) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

type ListBucketResponse_Item struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name string `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
}

func (x *ListBucketResponse_Item) Reset() {
	*x = ListBucketResponse_Item{}
	if protoimpl.UnsafeEnabled {
		mi := &file_s3_proto_msgTypes[11]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListBucketResponse_Item) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListBucketResponse_Item) ProtoMessage() {}

func (x *ListBucketResponse_Item) ProtoReflect() protoreflect.Message {
	mi := &file_s3_proto_msgTypes[11]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListBucketResponse_Item.ProtoReflect.Descriptor instead.
func (*ListBucketResponse_Item) Descriptor() ([]byte, []int) {
	return file_s3_proto_rawDescGZIP(), []int{3, 0}
}

func (x *ListBucketResponse_Item) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

var File_s3_proto protoreflect.FileDescriptor

var file_s3_proto_rawDesc = []byte{
	0x0a, 0x08, 0x73, 0x33, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1b, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x22, 0x5d, 0x0a, 0x06, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x12, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x08, 0x52, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x27, 0x0a, 0x0f, 0x65, 0x78, 0x70,
	0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x79, 0x73, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x05, 0x52, 0x0e, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61,
	0x79, 0x73, 0x22, 0x6a, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b,
	0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a,
	0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x70,
	0x75, 0x62, 0x6c, 0x69, 0x63, 0x12, 0x27, 0x0a, 0x0f, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x79, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e,
	0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61, 0x79, 0x73, 0x22, 0x2a,
	0x0a, 0x14, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x6b, 0x0a, 0x12, 0x4c, 0x69,
	0x73, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x12, 0x39, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
	0x23, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x69, 0x73,
	0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e,
	0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x1a, 0x1a, 0x0a, 0x04, 0x49,
	0x74, 0x65, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x76, 0x0a, 0x19, 0x50, 0x72, 0x65, 0x73, 0x69,
	0x67, 0x6e, 0x65, 0x64, 0x50, 0x75, 0x74, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05,
	0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74,
	0x6c, 0x65, 0x12, 0x2b, 0x0a, 0x03, 0x74, 0x74, 0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
	0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x22,
	0x46, 0x0a, 0x1a, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x75, 0x74, 0x4f,
	0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a,
	0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f,
	0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x09, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x22, 0x1f, 0x0a, 0x0b, 0x55, 0x72, 0x6c, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x22, 0xc7, 0x01, 0x0a, 0x19, 0x4f, 0x62, 0x6a,
	0x65, 0x63, 0x74, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x55, 0x72, 0x6c, 0x52,
	0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16,
	0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
	0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x26, 0x0a, 0x0c,
	0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01,
	0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70,
	0x65, 0x88, 0x01, 0x01, 0x12, 0x2b, 0x0a, 0x03, 0x74, 0x74, 0x6c, 0x18, 0x09, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x03, 0x74, 0x74,
	0x6c, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79,
	0x70, 0x65, 0x22, 0x9a, 0x01, 0x0a, 0x19, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x50, 0x65, 0x72,
	0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x12, 0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65,
	0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74,
	0x12, 0x14, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x26, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
	0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b,
	0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x88, 0x01, 0x01, 0x42, 0x0f,
	0x0a, 0x0d, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x22,
	0x45, 0x0a, 0x13, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52,
	0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16,
	0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
	0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x22, 0x29, 0x0a, 0x13, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65,
	0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a,
	0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
	0x65, 0x32, 0xce, 0x04, 0x0a, 0x02, 0x53, 0x33, 0x12, 0x53, 0x0a, 0x0c, 0x43, 0x72, 0x65, 0x61,
	0x74, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1f, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e,
	0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63, 0x6b,
	0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x63,
	0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x46, 0x0a,
	0x0a, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16, 0x2e, 0x67, 0x6f,
	0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d,
	0x70, 0x74, 0x79, 0x1a, 0x1e, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31,
	0x2e, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x49, 0x0a, 0x0c, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x42,
	0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1f, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e,
	0x76, 0x31, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52,
	0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x22, 0x00,
	0x12, 0x65, 0x0a, 0x12, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x75, 0x74,
	0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x25, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33,
	0x2e, 0x76, 0x31, 0x2e, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x75, 0x74,
	0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x72, 0x65, 0x73, 0x69,
	0x67, 0x6e, 0x65, 0x64, 0x50, 0x75, 0x74, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x56, 0x0a, 0x12, 0x4f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x50, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x12, 0x25, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x50, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x1a, 0x17, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76,
	0x31, 0x2e, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12,
	0x56, 0x0a, 0x12, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e,
	0x65, 0x64, 0x55, 0x72, 0x6c, 0x12, 0x25, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e,
	0x76, 0x31, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x50, 0x72, 0x65, 0x73, 0x69, 0x67, 0x6e,
	0x65, 0x64, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x17, 0x2e, 0x70,
	0x61, 0x6c, 0x6d, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x72, 0x6c, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x49, 0x0a, 0x0c, 0x52, 0x65, 0x6d, 0x6f, 0x76,
	0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x1f, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x73,
	0x33, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63,
	0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79,
	0x22, 0x00, 0x42, 0x5e, 0x0a, 0x28, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x78, 0x69, 0x76, 0x2e, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x73, 0x2e, 0x73, 0x33, 0x2e, 0x76, 0x31, 0x50, 0x01,
	0x5a, 0x30, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x61, 0x74,
	0x75, 0x72, 0x6e, 0x2d, 0x78, 0x69, 0x76, 0x2f, 0x70, 0x61, 0x6c, 0x6d, 0x2f, 0x61, 0x74, 0x72,
	0x6f, 0x70, 0x61, 0x2f, 0x73, 0x33, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x2f,
	0x76, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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

var file_s3_proto_msgTypes = make([]protoimpl.MessageInfo, 12)
var file_s3_proto_goTypes = []any{
	(*Bucket)(nil),                     // 0: palm.s3.v1.Bucket
	(*CreateBucketRequest)(nil),        // 1: palm.s3.v1.CreateBucketRequest
	(*CreateBucketResponse)(nil),       // 2: palm.s3.v1.CreateBucketResponse
	(*ListBucketResponse)(nil),         // 3: palm.s3.v1.ListBucketResponse
	(*PresignedPutObjectRequest)(nil),  // 4: palm.s3.v1.PresignedPutObjectRequest
	(*PresignedPutObjectResponse)(nil), // 5: palm.s3.v1.PresignedPutObjectResponse
	(*UrlResponse)(nil),                // 6: palm.s3.v1.UrlResponse
	(*ObjectPresignedUrlRequest)(nil),  // 7: palm.s3.v1.ObjectPresignedUrlRequest
	(*ObjectPermanentUrlRequest)(nil),  // 8: palm.s3.v1.ObjectPermanentUrlRequest
	(*RemoveObjectRequest)(nil),        // 9: palm.s3.v1.RemoveObjectRequest
	(*RemoveBucketRequest)(nil),        // 10: palm.s3.v1.RemoveBucketRequest
	(*ListBucketResponse_Item)(nil),    // 11: palm.s3.v1.ListBucketResponse.Item
	(*durationpb.Duration)(nil),        // 12: google.protobuf.Duration
	(*emptypb.Empty)(nil),              // 13: google.protobuf.Empty
}
var file_s3_proto_depIdxs = []int32{
	11, // 0: palm.s3.v1.ListBucketResponse.items:type_name -> palm.s3.v1.ListBucketResponse.Item
	12, // 1: palm.s3.v1.PresignedPutObjectRequest.ttl:type_name -> google.protobuf.Duration
	12, // 2: palm.s3.v1.ObjectPresignedUrlRequest.ttl:type_name -> google.protobuf.Duration
	1,  // 3: palm.s3.v1.S3.CreateBucket:input_type -> palm.s3.v1.CreateBucketRequest
	13, // 4: palm.s3.v1.S3.ListBucket:input_type -> google.protobuf.Empty
	10, // 5: palm.s3.v1.S3.RemoveBucket:input_type -> palm.s3.v1.RemoveBucketRequest
	4,  // 6: palm.s3.v1.S3.PresignedPutObject:input_type -> palm.s3.v1.PresignedPutObjectRequest
	8,  // 7: palm.s3.v1.S3.ObjectPermanentUrl:input_type -> palm.s3.v1.ObjectPermanentUrlRequest
	7,  // 8: palm.s3.v1.S3.ObjectPresignedUrl:input_type -> palm.s3.v1.ObjectPresignedUrlRequest
	9,  // 9: palm.s3.v1.S3.RemoveObject:input_type -> palm.s3.v1.RemoveObjectRequest
	2,  // 10: palm.s3.v1.S3.CreateBucket:output_type -> palm.s3.v1.CreateBucketResponse
	3,  // 11: palm.s3.v1.S3.ListBucket:output_type -> palm.s3.v1.ListBucketResponse
	13, // 12: palm.s3.v1.S3.RemoveBucket:output_type -> google.protobuf.Empty
	5,  // 13: palm.s3.v1.S3.PresignedPutObject:output_type -> palm.s3.v1.PresignedPutObjectResponse
	6,  // 14: palm.s3.v1.S3.ObjectPermanentUrl:output_type -> palm.s3.v1.UrlResponse
	6,  // 15: palm.s3.v1.S3.ObjectPresignedUrl:output_type -> palm.s3.v1.UrlResponse
	13, // 16: palm.s3.v1.S3.RemoveObject:output_type -> google.protobuf.Empty
	10, // [10:17] is the sub-list for method output_type
	3,  // [3:10] is the sub-list for method input_type
	3,  // [3:3] is the sub-list for extension type_name
	3,  // [3:3] is the sub-list for extension extendee
	0,  // [0:3] is the sub-list for field type_name
}

func init() { file_s3_proto_init() }
func file_s3_proto_init() {
	if File_s3_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_s3_proto_msgTypes[0].Exporter = func(v any, i int) any {
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
		file_s3_proto_msgTypes[1].Exporter = func(v any, i int) any {
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
		file_s3_proto_msgTypes[2].Exporter = func(v any, i int) any {
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
		file_s3_proto_msgTypes[3].Exporter = func(v any, i int) any {
			switch v := v.(*ListBucketResponse); i {
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
		file_s3_proto_msgTypes[4].Exporter = func(v any, i int) any {
			switch v := v.(*PresignedPutObjectRequest); i {
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
		file_s3_proto_msgTypes[5].Exporter = func(v any, i int) any {
			switch v := v.(*PresignedPutObjectResponse); i {
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
		file_s3_proto_msgTypes[6].Exporter = func(v any, i int) any {
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
		file_s3_proto_msgTypes[7].Exporter = func(v any, i int) any {
			switch v := v.(*ObjectPresignedUrlRequest); i {
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
		file_s3_proto_msgTypes[8].Exporter = func(v any, i int) any {
			switch v := v.(*ObjectPermanentUrlRequest); i {
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
		file_s3_proto_msgTypes[9].Exporter = func(v any, i int) any {
			switch v := v.(*RemoveObjectRequest); i {
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
		file_s3_proto_msgTypes[10].Exporter = func(v any, i int) any {
			switch v := v.(*RemoveBucketRequest); i {
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
		file_s3_proto_msgTypes[11].Exporter = func(v any, i int) any {
			switch v := v.(*ListBucketResponse_Item); i {
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
	file_s3_proto_msgTypes[7].OneofWrappers = []any{}
	file_s3_proto_msgTypes[8].OneofWrappers = []any{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_s3_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   12,
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
