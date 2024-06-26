// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v4.25.1
// source: minio.proto

package v2

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	S3_CreateBucket_FullMethodName    = "/palm.lilac.minio.v1.S3/CreateBucket"
	S3_UploadFile_FullMethodName      = "/palm.lilac.minio.v1.S3/UploadFile"
	S3_GetPresignedUrl_FullMethodName = "/palm.lilac.minio.v1.S3/GetPresignedUrl"
	S3_GetPermanentUrl_FullMethodName = "/palm.lilac.minio.v1.S3/GetPermanentUrl"
)

// S3Client is the client API for S3 service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type S3Client interface {
	CreateBucket(ctx context.Context, in *CreateBucketRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	UploadFile(ctx context.Context, in *UploadFileRequest, opts ...grpc.CallOption) (*UploadFileResponse, error)
	GetPresignedUrl(ctx context.Context, in *GetPresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
	GetPermanentUrl(ctx context.Context, in *GetPermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
}

type s3Client struct {
	cc grpc.ClientConnInterface
}

func NewS3Client(cc grpc.ClientConnInterface) S3Client {
	return &s3Client{cc}
}

func (c *s3Client) CreateBucket(ctx context.Context, in *CreateBucketRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, S3_CreateBucket_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) UploadFile(ctx context.Context, in *UploadFileRequest, opts ...grpc.CallOption) (*UploadFileResponse, error) {
	out := new(UploadFileResponse)
	err := c.cc.Invoke(ctx, S3_UploadFile_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) GetPresignedUrl(ctx context.Context, in *GetPresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_GetPresignedUrl_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) GetPermanentUrl(ctx context.Context, in *GetPermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_GetPermanentUrl_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// S3Server is the server API for S3 service.
// All implementations must embed UnimplementedS3Server
// for forward compatibility
type S3Server interface {
	CreateBucket(context.Context, *CreateBucketRequest) (*emptypb.Empty, error)
	UploadFile(context.Context, *UploadFileRequest) (*UploadFileResponse, error)
	GetPresignedUrl(context.Context, *GetPresignedUrlRequest) (*UrlResponse, error)
	GetPermanentUrl(context.Context, *GetPermanentUrlRequest) (*UrlResponse, error)
	mustEmbedUnimplementedS3Server()
}

// UnimplementedS3Server must be embedded to have forward compatible implementations.
type UnimplementedS3Server struct {
}

func (UnimplementedS3Server) CreateBucket(context.Context, *CreateBucketRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateBucket not implemented")
}
func (UnimplementedS3Server) UploadFile(context.Context, *UploadFileRequest) (*UploadFileResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UploadFile not implemented")
}
func (UnimplementedS3Server) GetPresignedUrl(context.Context, *GetPresignedUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPresignedUrl not implemented")
}
func (UnimplementedS3Server) GetPermanentUrl(context.Context, *GetPermanentUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPermanentUrl not implemented")
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

func _S3_UploadFile_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UploadFileRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).UploadFile(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_UploadFile_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).UploadFile(ctx, req.(*UploadFileRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_GetPresignedUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetPresignedUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).GetPresignedUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_GetPresignedUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).GetPresignedUrl(ctx, req.(*GetPresignedUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_GetPermanentUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetPermanentUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).GetPermanentUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_GetPermanentUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).GetPermanentUrl(ctx, req.(*GetPermanentUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// S3_ServiceDesc is the grpc.ServiceDesc for S3 service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var S3_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.lilac.minio.v1.S3",
	HandlerType: (*S3Server)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "CreateBucket",
			Handler:    _S3_CreateBucket_Handler,
		},
		{
			MethodName: "UploadFile",
			Handler:    _S3_UploadFile_Handler,
		},
		{
			MethodName: "GetPresignedUrl",
			Handler:    _S3_GetPresignedUrl_Handler,
		},
		{
			MethodName: "GetPermanentUrl",
			Handler:    _S3_GetPermanentUrl_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "minio.proto",
}
