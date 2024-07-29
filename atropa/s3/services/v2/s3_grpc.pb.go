// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v3.21.12
// source: s3.proto

package v2

import (
	context "context"
	v2 "github.com/saturn-xiv/palm/atropa/balsam/services/v2"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.62.0 or later.
const _ = grpc.SupportPackageIsVersion8

const (
	S3_CreateBucket_FullMethodName = "/palm.s3.v1.S3/CreateBucket"
	S3_Upload_FullMethodName       = "/palm.s3.v1.S3/Upload"
	S3_PermanentUrl_FullMethodName = "/palm.s3.v1.S3/PermanentUrl"
	S3_PresignedUrl_FullMethodName = "/palm.s3.v1.S3/PresignedUrl"
)

// S3Client is the client API for S3 service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type S3Client interface {
	CreateBucket(ctx context.Context, in *CreateBucketRequest, opts ...grpc.CallOption) (*CreateBucketResponse, error)
	Upload(ctx context.Context, in *UploadRequest, opts ...grpc.CallOption) (*UploadResponse, error)
	PermanentUrl(ctx context.Context, in *PermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
	PresignedUrl(ctx context.Context, in *PresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
}

type s3Client struct {
	cc grpc.ClientConnInterface
}

func NewS3Client(cc grpc.ClientConnInterface) S3Client {
	return &s3Client{cc}
}

func (c *s3Client) CreateBucket(ctx context.Context, in *CreateBucketRequest, opts ...grpc.CallOption) (*CreateBucketResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(CreateBucketResponse)
	err := c.cc.Invoke(ctx, S3_CreateBucket_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) Upload(ctx context.Context, in *UploadRequest, opts ...grpc.CallOption) (*UploadResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UploadResponse)
	err := c.cc.Invoke(ctx, S3_Upload_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) PermanentUrl(ctx context.Context, in *PermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_PermanentUrl_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) PresignedUrl(ctx context.Context, in *PresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_PresignedUrl_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// S3Server is the server API for S3 service.
// All implementations must embed UnimplementedS3Server
// for forward compatibility
type S3Server interface {
	CreateBucket(context.Context, *CreateBucketRequest) (*CreateBucketResponse, error)
	Upload(context.Context, *UploadRequest) (*UploadResponse, error)
	PermanentUrl(context.Context, *PermanentUrlRequest) (*UrlResponse, error)
	PresignedUrl(context.Context, *PresignedUrlRequest) (*UrlResponse, error)
	mustEmbedUnimplementedS3Server()
}

// UnimplementedS3Server must be embedded to have forward compatible implementations.
type UnimplementedS3Server struct {
}

func (UnimplementedS3Server) CreateBucket(context.Context, *CreateBucketRequest) (*CreateBucketResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateBucket not implemented")
}
func (UnimplementedS3Server) Upload(context.Context, *UploadRequest) (*UploadResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Upload not implemented")
}
func (UnimplementedS3Server) PermanentUrl(context.Context, *PermanentUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PermanentUrl not implemented")
}
func (UnimplementedS3Server) PresignedUrl(context.Context, *PresignedUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PresignedUrl not implemented")
}
func (UnimplementedS3Server) mustEmbedUnimplementedS3Server() {}

// UnsafeS3Server may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to S3Server will
// result in compilation errors.
type UnsafeS3Server interface {
	mustEmbedUnimplementedS3Server()
}

func RegisterS3Server(s grpc.ServiceRegistrar, srv S3Server) {
	s.RegisterService(&S3_ServiceDesc, srv)
}

func _S3_CreateBucket_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateBucketRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).CreateBucket(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_CreateBucket_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).CreateBucket(ctx, req.(*CreateBucketRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_Upload_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UploadRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).Upload(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_Upload_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).Upload(ctx, req.(*UploadRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_PermanentUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PermanentUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).PermanentUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_PermanentUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).PermanentUrl(ctx, req.(*PermanentUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_PresignedUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PresignedUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).PresignedUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_PresignedUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).PresignedUrl(ctx, req.(*PresignedUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// S3_ServiceDesc is the grpc.ServiceDesc for S3 service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var S3_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.s3.v1.S3",
	HandlerType: (*S3Server)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "CreateBucket",
			Handler:    _S3_CreateBucket_Handler,
		},
		{
			MethodName: "Upload",
			Handler:    _S3_Upload_Handler,
		},
		{
			MethodName: "PermanentUrl",
			Handler:    _S3_PermanentUrl_Handler,
		},
		{
			MethodName: "PresignedUrl",
			Handler:    _S3_PresignedUrl_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "s3.proto",
}

const (
	Attachment_Disable_FullMethodName       = "/palm.s3.v1.Attachment/Disable"
	Attachment_Index_FullMethodName         = "/palm.s3.v1.Attachment/Index"
	Attachment_SetTitle_FullMethodName      = "/palm.s3.v1.Attachment/SetTitle"
	Attachment_ById_FullMethodName          = "/palm.s3.v1.Attachment/ById"
	Attachment_ByUser_FullMethodName        = "/palm.s3.v1.Attachment/ByUser"
	Attachment_Clear_FullMethodName         = "/palm.s3.v1.Attachment/Clear"
	Attachment_ByResource_FullMethodName    = "/palm.s3.v1.Attachment/ByResource"
	Attachment_Create_FullMethodName        = "/palm.s3.v1.Attachment/Create"
	Attachment_SetUploadedAt_FullMethodName = "/palm.s3.v1.Attachment/SetUploadedAt"
)

// AttachmentClient is the client API for Attachment service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type AttachmentClient interface {
	Disable(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	Index(ctx context.Context, in *v2.Pager, opts ...grpc.CallOption) (*AttachmentIndexResponse, error)
	SetTitle(ctx context.Context, in *AttachmentSetTitleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	ById(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*AttachmentIndexResponse_Item, error)
	ByUser(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*AttachmentIndexResponse, error)
	Clear(ctx context.Context, in *emptypb.Empty, opts ...grpc.CallOption) (*emptypb.Empty, error)
	ByResource(ctx context.Context, in *v2.ResourceRequest, opts ...grpc.CallOption) (*AttachmentListResponse, error)
	Create(ctx context.Context, in *AttachmentCreateRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	SetUploadedAt(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
}

type attachmentClient struct {
	cc grpc.ClientConnInterface
}

func NewAttachmentClient(cc grpc.ClientConnInterface) AttachmentClient {
	return &attachmentClient{cc}
}

func (c *attachmentClient) Disable(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Attachment_Disable_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) Index(ctx context.Context, in *v2.Pager, opts ...grpc.CallOption) (*AttachmentIndexResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AttachmentIndexResponse)
	err := c.cc.Invoke(ctx, Attachment_Index_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) SetTitle(ctx context.Context, in *AttachmentSetTitleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Attachment_SetTitle_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) ById(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*AttachmentIndexResponse_Item, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AttachmentIndexResponse_Item)
	err := c.cc.Invoke(ctx, Attachment_ById_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) ByUser(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*AttachmentIndexResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AttachmentIndexResponse)
	err := c.cc.Invoke(ctx, Attachment_ByUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) Clear(ctx context.Context, in *emptypb.Empty, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Attachment_Clear_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) ByResource(ctx context.Context, in *v2.ResourceRequest, opts ...grpc.CallOption) (*AttachmentListResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AttachmentListResponse)
	err := c.cc.Invoke(ctx, Attachment_ByResource_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) Create(ctx context.Context, in *AttachmentCreateRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Attachment_Create_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *attachmentClient) SetUploadedAt(ctx context.Context, in *v2.IdRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Attachment_SetUploadedAt_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// AttachmentServer is the server API for Attachment service.
// All implementations must embed UnimplementedAttachmentServer
// for forward compatibility
type AttachmentServer interface {
	Disable(context.Context, *v2.IdRequest) (*emptypb.Empty, error)
	Index(context.Context, *v2.Pager) (*AttachmentIndexResponse, error)
	SetTitle(context.Context, *AttachmentSetTitleRequest) (*emptypb.Empty, error)
	ById(context.Context, *v2.IdRequest) (*AttachmentIndexResponse_Item, error)
	ByUser(context.Context, *v2.IdRequest) (*AttachmentIndexResponse, error)
	Clear(context.Context, *emptypb.Empty) (*emptypb.Empty, error)
	ByResource(context.Context, *v2.ResourceRequest) (*AttachmentListResponse, error)
	Create(context.Context, *AttachmentCreateRequest) (*emptypb.Empty, error)
	SetUploadedAt(context.Context, *v2.IdRequest) (*emptypb.Empty, error)
	mustEmbedUnimplementedAttachmentServer()
}

// UnimplementedAttachmentServer must be embedded to have forward compatible implementations.
type UnimplementedAttachmentServer struct {
}

func (UnimplementedAttachmentServer) Disable(context.Context, *v2.IdRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Disable not implemented")
}
func (UnimplementedAttachmentServer) Index(context.Context, *v2.Pager) (*AttachmentIndexResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Index not implemented")
}
func (UnimplementedAttachmentServer) SetTitle(context.Context, *AttachmentSetTitleRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SetTitle not implemented")
}
func (UnimplementedAttachmentServer) ById(context.Context, *v2.IdRequest) (*AttachmentIndexResponse_Item, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ById not implemented")
}
func (UnimplementedAttachmentServer) ByUser(context.Context, *v2.IdRequest) (*AttachmentIndexResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ByUser not implemented")
}
func (UnimplementedAttachmentServer) Clear(context.Context, *emptypb.Empty) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Clear not implemented")
}
func (UnimplementedAttachmentServer) ByResource(context.Context, *v2.ResourceRequest) (*AttachmentListResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ByResource not implemented")
}
func (UnimplementedAttachmentServer) Create(context.Context, *AttachmentCreateRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Create not implemented")
}
func (UnimplementedAttachmentServer) SetUploadedAt(context.Context, *v2.IdRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SetUploadedAt not implemented")
}
func (UnimplementedAttachmentServer) mustEmbedUnimplementedAttachmentServer() {}

// UnsafeAttachmentServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to AttachmentServer will
// result in compilation errors.
type UnsafeAttachmentServer interface {
	mustEmbedUnimplementedAttachmentServer()
}

func RegisterAttachmentServer(s grpc.ServiceRegistrar, srv AttachmentServer) {
	s.RegisterService(&Attachment_ServiceDesc, srv)
}

func _Attachment_Disable_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.IdRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).Disable(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_Disable_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).Disable(ctx, req.(*v2.IdRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_Index_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.Pager)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).Index(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_Index_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).Index(ctx, req.(*v2.Pager))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_SetTitle_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AttachmentSetTitleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).SetTitle(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_SetTitle_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).SetTitle(ctx, req.(*AttachmentSetTitleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_ById_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.IdRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).ById(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_ById_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).ById(ctx, req.(*v2.IdRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_ByUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.IdRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).ByUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_ByUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).ByUser(ctx, req.(*v2.IdRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_Clear_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(emptypb.Empty)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).Clear(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_Clear_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).Clear(ctx, req.(*emptypb.Empty))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_ByResource_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.ResourceRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).ByResource(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_ByResource_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).ByResource(ctx, req.(*v2.ResourceRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_Create_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AttachmentCreateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).Create(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_Create_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).Create(ctx, req.(*AttachmentCreateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Attachment_SetUploadedAt_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(v2.IdRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AttachmentServer).SetUploadedAt(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Attachment_SetUploadedAt_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AttachmentServer).SetUploadedAt(ctx, req.(*v2.IdRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Attachment_ServiceDesc is the grpc.ServiceDesc for Attachment service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Attachment_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.s3.v1.Attachment",
	HandlerType: (*AttachmentServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Disable",
			Handler:    _Attachment_Disable_Handler,
		},
		{
			MethodName: "Index",
			Handler:    _Attachment_Index_Handler,
		},
		{
			MethodName: "SetTitle",
			Handler:    _Attachment_SetTitle_Handler,
		},
		{
			MethodName: "ById",
			Handler:    _Attachment_ById_Handler,
		},
		{
			MethodName: "ByUser",
			Handler:    _Attachment_ByUser_Handler,
		},
		{
			MethodName: "Clear",
			Handler:    _Attachment_Clear_Handler,
		},
		{
			MethodName: "ByResource",
			Handler:    _Attachment_ByResource_Handler,
		},
		{
			MethodName: "Create",
			Handler:    _Attachment_Create_Handler,
		},
		{
			MethodName: "SetUploadedAt",
			Handler:    _Attachment_SetUploadedAt_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "s3.proto",
}
