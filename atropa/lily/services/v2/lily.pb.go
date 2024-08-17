// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.34.2
// 	protoc        v5.27.2
// source: lily.proto

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

// https://owl.purdue.edu/owl/general_writing/general_writing_faqs.html
type Style int32

const (
	// https://apastyle.apa.org/
	// https://owl.purdue.edu/owl/research_and_citation/apa_style/apa_formatting_and_style_guide/general_format.html
	Style_APA Style = 0
	// https://www.chicagomanualofstyle.org/home.html
	// https://owl.purdue.edu/owl/research_and_citation/chicago_manual_17th_edition/cmos_formatting_and_style_guide/chicago_manual_of_style_17th_edition.html
	Style_CMOS Style = 1
	// https://www.mla.org/MLA-Style
	// https://owl.purdue.edu/owl/research_and_citation/mla_style/mla_formatting_and_style_guide/mla_formatting_and_style_guide.html#:~:text=MLA%20(Modern%20Language%20Association)%20style,the%20liberal%20arts%20and%20humanities.
	Style_MLA Style = 2
)

// Enum value maps for Style.
var (
	Style_name = map[int32]string{
		0: "APA",
		1: "CMOS",
		2: "MLA",
	}
	Style_value = map[string]int32{
		"APA":  0,
		"CMOS": 1,
		"MLA":  2,
	}
)

func (x Style) Enum() *Style {
	p := new(Style)
	*p = x
	return p
}

func (x Style) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Style) Descriptor() protoreflect.EnumDescriptor {
	return file_lily_proto_enumTypes[0].Descriptor()
}

func (Style) Type() protoreflect.EnumType {
	return &file_lily_proto_enumTypes[0]
}

func (x Style) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Style.Descriptor instead.
func (Style) EnumDescriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{0}
}

type Format int32

const (
	Format_Pdf  Format = 0
	Format_Word Format = 1
)

// Enum value maps for Format.
var (
	Format_name = map[int32]string{
		0: "Pdf",
		1: "Word",
	}
	Format_value = map[string]int32{
		"Pdf":  0,
		"Word": 1,
	}
)

func (x Format) Enum() *Format {
	p := new(Format)
	*p = x
	return p
}

func (x Format) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Format) Descriptor() protoreflect.EnumDescriptor {
	return file_lily_proto_enumTypes[1].Descriptor()
}

func (Format) Type() protoreflect.EnumType {
	return &file_lily_proto_enumTypes[1]
}

func (x Format) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Format.Descriptor instead.
func (Format) EnumDescriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{1}
}

type File struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Bucket string `protobuf:"bytes,1,opt,name=bucket,proto3" json:"bucket,omitempty"`
	Object string `protobuf:"bytes,2,opt,name=object,proto3" json:"object,omitempty"`
}

func (x *File) Reset() {
	*x = File{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *File) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*File) ProtoMessage() {}

func (x *File) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use File.ProtoReflect.Descriptor instead.
func (*File) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{0}
}

func (x *File) GetBucket() string {
	if x != nil {
		return x.Bucket
	}
	return ""
}

func (x *File) GetObject() string {
	if x != nil {
		return x.Object
	}
	return ""
}

type Book struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *Book) Reset() {
	*x = Book{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Book) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Book) ProtoMessage() {}

func (x *Book) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Book.ProtoReflect.Descriptor instead.
func (*Book) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{1}
}

type Slideshow struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *Slideshow) Reset() {
	*x = Slideshow{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Slideshow) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Slideshow) ProtoMessage() {}

func (x *Slideshow) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Slideshow.ProtoReflect.Descriptor instead.
func (*Slideshow) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{2}
}

type Article struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *Article) Reset() {
	*x = Article{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Article) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Article) ProtoMessage() {}

func (x *Article) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Article.ProtoReflect.Descriptor instead.
func (*Article) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{3}
}

type Section struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Items []*Section `protobuf:"bytes,1,rep,name=items,proto3" json:"items,omitempty"`
}

func (x *Section) Reset() {
	*x = Section{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Section) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Section) ProtoMessage() {}

func (x *Section) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Section.ProtoReflect.Descriptor instead.
func (*Section) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{4}
}

func (x *Section) GetItems() []*Section {
	if x != nil {
		return x.Items
	}
	return nil
}

type TexBuildTask struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Entry       string            `protobuf:"bytes,1,opt,name=entry,proto3" json:"entry,omitempty"`
	Attachments map[string][]byte `protobuf:"bytes,2,rep,name=attachments,proto3" json:"attachments,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
	Output      *File             `protobuf:"bytes,9,opt,name=output,proto3" json:"output,omitempty"`
}

func (x *TexBuildTask) Reset() {
	*x = TexBuildTask{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TexBuildTask) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TexBuildTask) ProtoMessage() {}

func (x *TexBuildTask) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TexBuildTask.ProtoReflect.Descriptor instead.
func (*TexBuildTask) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{5}
}

func (x *TexBuildTask) GetEntry() string {
	if x != nil {
		return x.Entry
	}
	return ""
}

func (x *TexBuildTask) GetAttachments() map[string][]byte {
	if x != nil {
		return x.Attachments
	}
	return nil
}

func (x *TexBuildTask) GetOutput() *File {
	if x != nil {
		return x.Output
	}
	return nil
}

type TexRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Payload:
	//
	//	*TexRequest_Book
	//	*TexRequest_Article
	//	*TexRequest_Slideshow
	Payload     isTexRequest_Payload `protobuf_oneof:"Payload"`
	Attachments map[string][]byte    `protobuf:"bytes,97,rep,name=attachments,proto3" json:"attachments,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
	Style       Style                `protobuf:"varint,98,opt,name=style,proto3,enum=palm.lily.v1.Style" json:"style,omitempty"`
	Format      Format               `protobuf:"varint,99,opt,name=format,proto3,enum=palm.lily.v1.Format" json:"format,omitempty"`
}

func (x *TexRequest) Reset() {
	*x = TexRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TexRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TexRequest) ProtoMessage() {}

func (x *TexRequest) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TexRequest.ProtoReflect.Descriptor instead.
func (*TexRequest) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{6}
}

func (m *TexRequest) GetPayload() isTexRequest_Payload {
	if m != nil {
		return m.Payload
	}
	return nil
}

func (x *TexRequest) GetBook() *Book {
	if x, ok := x.GetPayload().(*TexRequest_Book); ok {
		return x.Book
	}
	return nil
}

func (x *TexRequest) GetArticle() *Article {
	if x, ok := x.GetPayload().(*TexRequest_Article); ok {
		return x.Article
	}
	return nil
}

func (x *TexRequest) GetSlideshow() *Slideshow {
	if x, ok := x.GetPayload().(*TexRequest_Slideshow); ok {
		return x.Slideshow
	}
	return nil
}

func (x *TexRequest) GetAttachments() map[string][]byte {
	if x != nil {
		return x.Attachments
	}
	return nil
}

func (x *TexRequest) GetStyle() Style {
	if x != nil {
		return x.Style
	}
	return Style_APA
}

func (x *TexRequest) GetFormat() Format {
	if x != nil {
		return x.Format
	}
	return Format_Pdf
}

type isTexRequest_Payload interface {
	isTexRequest_Payload()
}

type TexRequest_Book struct {
	Book *Book `protobuf:"bytes,1,opt,name=book,proto3,oneof"`
}

type TexRequest_Article struct {
	Article *Article `protobuf:"bytes,2,opt,name=article,proto3,oneof"`
}

type TexRequest_Slideshow struct {
	Slideshow *Slideshow `protobuf:"bytes,3,opt,name=slideshow,proto3,oneof"`
}

func (*TexRequest_Book) isTexRequest_Payload() {}

func (*TexRequest_Article) isTexRequest_Payload() {}

func (*TexRequest_Slideshow) isTexRequest_Payload() {}

type ShowRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	File *File                `protobuf:"bytes,1,opt,name=file,proto3" json:"file,omitempty"`
	Ttl  *durationpb.Duration `protobuf:"bytes,9,opt,name=ttl,proto3" json:"ttl,omitempty"`
}

func (x *ShowRequest) Reset() {
	*x = ShowRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ShowRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ShowRequest) ProtoMessage() {}

func (x *ShowRequest) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ShowRequest.ProtoReflect.Descriptor instead.
func (*ShowRequest) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{7}
}

func (x *ShowRequest) GetFile() *File {
	if x != nil {
		return x.File
	}
	return nil
}

func (x *ShowRequest) GetTtl() *durationpb.Duration {
	if x != nil {
		return x.Ttl
	}
	return nil
}

type ShowResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Url string `protobuf:"bytes,1,opt,name=url,proto3" json:"url,omitempty"`
}

func (x *ShowResponse) Reset() {
	*x = ShowResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ShowResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ShowResponse) ProtoMessage() {}

func (x *ShowResponse) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ShowResponse.ProtoReflect.Descriptor instead.
func (*ShowResponse) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{8}
}

func (x *ShowResponse) GetUrl() string {
	if x != nil {
		return x.Url
	}
	return ""
}

type StatusResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Log     string `protobuf:"bytes,1,opt,name=log,proto3" json:"log,omitempty"`
	Succeed bool   `protobuf:"varint,2,opt,name=succeed,proto3" json:"succeed,omitempty"`
}

func (x *StatusResponse) Reset() {
	*x = StatusResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_lily_proto_msgTypes[9]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *StatusResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*StatusResponse) ProtoMessage() {}

func (x *StatusResponse) ProtoReflect() protoreflect.Message {
	mi := &file_lily_proto_msgTypes[9]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use StatusResponse.ProtoReflect.Descriptor instead.
func (*StatusResponse) Descriptor() ([]byte, []int) {
	return file_lily_proto_rawDescGZIP(), []int{9}
}

func (x *StatusResponse) GetLog() string {
	if x != nil {
		return x.Log
	}
	return ""
}

func (x *StatusResponse) GetSucceed() bool {
	if x != nil {
		return x.Succeed
	}
	return false
}

var File_lily_proto protoreflect.FileDescriptor

var file_lily_proto_rawDesc = []byte{
	0x0a, 0x0a, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x70, 0x61,
	0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x36, 0x0a, 0x04, 0x46, 0x69,
	0x6c, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x62,
	0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x62, 0x6a, 0x65,
	0x63, 0x74, 0x22, 0x06, 0x0a, 0x04, 0x42, 0x6f, 0x6f, 0x6b, 0x22, 0x0b, 0x0a, 0x09, 0x53, 0x6c,
	0x69, 0x64, 0x65, 0x73, 0x68, 0x6f, 0x77, 0x22, 0x09, 0x0a, 0x07, 0x41, 0x72, 0x74, 0x69, 0x63,
	0x6c, 0x65, 0x22, 0x36, 0x0a, 0x07, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a,
	0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70,
	0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65, 0x63, 0x74,
	0x69, 0x6f, 0x6e, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x22, 0xdf, 0x01, 0x0a, 0x0c, 0x54,
	0x65, 0x78, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x14, 0x0a, 0x05, 0x65,
	0x6e, 0x74, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6e, 0x74, 0x72,
	0x79, 0x12, 0x4d, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73,
	0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69,
	0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x65, 0x78, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x54, 0x61,
	0x73, 0x6b, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x45, 0x6e,
	0x74, 0x72, 0x79, 0x52, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73,
	0x12, 0x2a, 0x0a, 0x06, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x12, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e,
	0x46, 0x69, 0x6c, 0x65, 0x52, 0x06, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x1a, 0x3e, 0x0a, 0x10,
	0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79,
	0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b,
	0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0x93, 0x03, 0x0a,
	0x0a, 0x54, 0x65, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x28, 0x0a, 0x04, 0x62,
	0x6f, 0x6f, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x42, 0x6f, 0x6f, 0x6b, 0x48, 0x00, 0x52,
	0x04, 0x62, 0x6f, 0x6f, 0x6b, 0x12, 0x31, 0x0a, 0x07, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69,
	0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x48, 0x00, 0x52,
	0x07, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x37, 0x0a, 0x09, 0x73, 0x6c, 0x69, 0x64,
	0x65, 0x73, 0x68, 0x6f, 0x77, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x61,
	0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x6c, 0x69, 0x64, 0x65,
	0x73, 0x68, 0x6f, 0x77, 0x48, 0x00, 0x52, 0x09, 0x73, 0x6c, 0x69, 0x64, 0x65, 0x73, 0x68, 0x6f,
	0x77, 0x12, 0x4b, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73,
	0x18, 0x61, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69,
	0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x65, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72,
	0x79, 0x52, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x29,
	0x0a, 0x05, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x18, 0x62, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x79,
	0x6c, 0x65, 0x52, 0x05, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x12, 0x2c, 0x0a, 0x06, 0x66, 0x6f, 0x72,
	0x6d, 0x61, 0x74, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x52,
	0x06, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x1a, 0x3e, 0x0a, 0x10, 0x41, 0x74, 0x74, 0x61, 0x63,
	0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b,
	0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a,
	0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61,
	0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x42, 0x09, 0x0a, 0x07, 0x50, 0x61, 0x79, 0x6c, 0x6f,
	0x61, 0x64, 0x22, 0x62, 0x0a, 0x0b, 0x53, 0x68, 0x6f, 0x77, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x12, 0x26, 0x0a, 0x04, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x12, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x46,
	0x69, 0x6c, 0x65, 0x52, 0x04, 0x66, 0x69, 0x6c, 0x65, 0x12, 0x2b, 0x0a, 0x03, 0x74, 0x74, 0x6c,
	0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x22, 0x20, 0x0a, 0x0c, 0x53, 0x68, 0x6f, 0x77, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x22, 0x3c, 0x0a, 0x0e, 0x53, 0x74, 0x61, 0x74,
	0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x6f,
	0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6c, 0x6f, 0x67, 0x12, 0x18, 0x0a, 0x07,
	0x73, 0x75, 0x63, 0x63, 0x65, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73,
	0x75, 0x63, 0x63, 0x65, 0x65, 0x64, 0x2a, 0x23, 0x0a, 0x05, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x12,
	0x07, 0x0a, 0x03, 0x41, 0x50, 0x41, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x43, 0x4d, 0x4f, 0x53,
	0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x4d, 0x4c, 0x41, 0x10, 0x02, 0x2a, 0x1b, 0x0a, 0x06, 0x46,
	0x6f, 0x72, 0x6d, 0x61, 0x74, 0x12, 0x07, 0x0a, 0x03, 0x50, 0x64, 0x66, 0x10, 0x00, 0x12, 0x08,
	0x0a, 0x04, 0x57, 0x6f, 0x72, 0x64, 0x10, 0x01, 0x32, 0xf7, 0x01, 0x0a, 0x03, 0x54, 0x65, 0x78,
	0x12, 0x38, 0x0a, 0x06, 0x54, 0x6f, 0x57, 0x6f, 0x72, 0x64, 0x12, 0x18, 0x2e, 0x70, 0x61, 0x6c,
	0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x65, 0x78, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79,
	0x2e, 0x76, 0x31, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x22, 0x00, 0x12, 0x37, 0x0a, 0x05, 0x54, 0x6f,
	0x50, 0x64, 0x66, 0x12, 0x18, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e,
	0x76, 0x31, 0x2e, 0x54, 0x65, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e,
	0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x69, 0x6c,
	0x65, 0x22, 0x00, 0x12, 0x3f, 0x0a, 0x04, 0x53, 0x68, 0x6f, 0x77, 0x12, 0x19, 0x2e, 0x70, 0x61,
	0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x6f, 0x77, 0x52,
	0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1a, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69,
	0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x6f, 0x77, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x22, 0x00, 0x12, 0x3c, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x12,
	0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x69,
	0x6c, 0x65, 0x1a, 0x1c, 0x2e, 0x70, 0x61, 0x6c, 0x6d, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76,
	0x31, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x22, 0x00, 0x42, 0x62, 0x0a, 0x2a, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x73, 0x61, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x78, 0x69, 0x76, 0x2e, 0x70, 0x61, 0x6c, 0x6d,
	0x2e, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x73, 0x2e, 0x6c, 0x69, 0x6c, 0x79, 0x2e, 0x76, 0x31,
	0x50, 0x01, 0x5a, 0x32, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73,
	0x61, 0x74, 0x75, 0x72, 0x6e, 0x2d, 0x78, 0x69, 0x76, 0x2f, 0x70, 0x61, 0x6c, 0x6d, 0x2f, 0x61,
	0x74, 0x72, 0x6f, 0x70, 0x61, 0x2f, 0x6c, 0x69, 0x6c, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x73, 0x2f, 0x76, 0x32, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_lily_proto_rawDescOnce sync.Once
	file_lily_proto_rawDescData = file_lily_proto_rawDesc
)

func file_lily_proto_rawDescGZIP() []byte {
	file_lily_proto_rawDescOnce.Do(func() {
		file_lily_proto_rawDescData = protoimpl.X.CompressGZIP(file_lily_proto_rawDescData)
	})
	return file_lily_proto_rawDescData
}

var file_lily_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_lily_proto_msgTypes = make([]protoimpl.MessageInfo, 12)
var file_lily_proto_goTypes = []any{
	(Style)(0),                  // 0: palm.lily.v1.Style
	(Format)(0),                 // 1: palm.lily.v1.Format
	(*File)(nil),                // 2: palm.lily.v1.File
	(*Book)(nil),                // 3: palm.lily.v1.Book
	(*Slideshow)(nil),           // 4: palm.lily.v1.Slideshow
	(*Article)(nil),             // 5: palm.lily.v1.Article
	(*Section)(nil),             // 6: palm.lily.v1.Section
	(*TexBuildTask)(nil),        // 7: palm.lily.v1.TexBuildTask
	(*TexRequest)(nil),          // 8: palm.lily.v1.TexRequest
	(*ShowRequest)(nil),         // 9: palm.lily.v1.ShowRequest
	(*ShowResponse)(nil),        // 10: palm.lily.v1.ShowResponse
	(*StatusResponse)(nil),      // 11: palm.lily.v1.StatusResponse
	nil,                         // 12: palm.lily.v1.TexBuildTask.AttachmentsEntry
	nil,                         // 13: palm.lily.v1.TexRequest.AttachmentsEntry
	(*durationpb.Duration)(nil), // 14: google.protobuf.Duration
}
var file_lily_proto_depIdxs = []int32{
	6,  // 0: palm.lily.v1.Section.items:type_name -> palm.lily.v1.Section
	12, // 1: palm.lily.v1.TexBuildTask.attachments:type_name -> palm.lily.v1.TexBuildTask.AttachmentsEntry
	2,  // 2: palm.lily.v1.TexBuildTask.output:type_name -> palm.lily.v1.File
	3,  // 3: palm.lily.v1.TexRequest.book:type_name -> palm.lily.v1.Book
	5,  // 4: palm.lily.v1.TexRequest.article:type_name -> palm.lily.v1.Article
	4,  // 5: palm.lily.v1.TexRequest.slideshow:type_name -> palm.lily.v1.Slideshow
	13, // 6: palm.lily.v1.TexRequest.attachments:type_name -> palm.lily.v1.TexRequest.AttachmentsEntry
	0,  // 7: palm.lily.v1.TexRequest.style:type_name -> palm.lily.v1.Style
	1,  // 8: palm.lily.v1.TexRequest.format:type_name -> palm.lily.v1.Format
	2,  // 9: palm.lily.v1.ShowRequest.file:type_name -> palm.lily.v1.File
	14, // 10: palm.lily.v1.ShowRequest.ttl:type_name -> google.protobuf.Duration
	8,  // 11: palm.lily.v1.Tex.ToWord:input_type -> palm.lily.v1.TexRequest
	8,  // 12: palm.lily.v1.Tex.ToPdf:input_type -> palm.lily.v1.TexRequest
	9,  // 13: palm.lily.v1.Tex.Show:input_type -> palm.lily.v1.ShowRequest
	2,  // 14: palm.lily.v1.Tex.Status:input_type -> palm.lily.v1.File
	2,  // 15: palm.lily.v1.Tex.ToWord:output_type -> palm.lily.v1.File
	2,  // 16: palm.lily.v1.Tex.ToPdf:output_type -> palm.lily.v1.File
	10, // 17: palm.lily.v1.Tex.Show:output_type -> palm.lily.v1.ShowResponse
	11, // 18: palm.lily.v1.Tex.Status:output_type -> palm.lily.v1.StatusResponse
	15, // [15:19] is the sub-list for method output_type
	11, // [11:15] is the sub-list for method input_type
	11, // [11:11] is the sub-list for extension type_name
	11, // [11:11] is the sub-list for extension extendee
	0,  // [0:11] is the sub-list for field type_name
}

func init() { file_lily_proto_init() }
func file_lily_proto_init() {
	if File_lily_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_lily_proto_msgTypes[0].Exporter = func(v any, i int) any {
			switch v := v.(*File); i {
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
		file_lily_proto_msgTypes[1].Exporter = func(v any, i int) any {
			switch v := v.(*Book); i {
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
		file_lily_proto_msgTypes[2].Exporter = func(v any, i int) any {
			switch v := v.(*Slideshow); i {
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
		file_lily_proto_msgTypes[3].Exporter = func(v any, i int) any {
			switch v := v.(*Article); i {
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
		file_lily_proto_msgTypes[4].Exporter = func(v any, i int) any {
			switch v := v.(*Section); i {
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
		file_lily_proto_msgTypes[5].Exporter = func(v any, i int) any {
			switch v := v.(*TexBuildTask); i {
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
		file_lily_proto_msgTypes[6].Exporter = func(v any, i int) any {
			switch v := v.(*TexRequest); i {
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
		file_lily_proto_msgTypes[7].Exporter = func(v any, i int) any {
			switch v := v.(*ShowRequest); i {
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
		file_lily_proto_msgTypes[8].Exporter = func(v any, i int) any {
			switch v := v.(*ShowResponse); i {
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
		file_lily_proto_msgTypes[9].Exporter = func(v any, i int) any {
			switch v := v.(*StatusResponse); i {
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
	file_lily_proto_msgTypes[6].OneofWrappers = []any{
		(*TexRequest_Book)(nil),
		(*TexRequest_Article)(nil),
		(*TexRequest_Slideshow)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_lily_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   12,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_lily_proto_goTypes,
		DependencyIndexes: file_lily_proto_depIdxs,
		EnumInfos:         file_lily_proto_enumTypes,
		MessageInfos:      file_lily_proto_msgTypes,
	}.Build()
	File_lily_proto = out.File
	file_lily_proto_rawDesc = nil
	file_lily_proto_goTypes = nil
	file_lily_proto_depIdxs = nil
}
