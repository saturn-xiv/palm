// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v3.21.12
// source: s3.proto

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
// Requires gRPC-Go v1.62.0 or later.
const _ = grpc.SupportPackageIsVersion8

const (
	S3_CreateBucket_FullMethodName       = "/palm.s3.v1.S3/CreateBucket"
	S3_UploadObject_FullMethodName       = "/palm.s3.v1.S3/UploadObject"
	S3_ObjectPermanentUrl_FullMethodName = "/palm.s3.v1.S3/ObjectPermanentUrl"
	S3_ObjectPresignedUrl_FullMethodName = "/palm.s3.v1.S3/ObjectPresignedUrl"
	S3_RemoveObject_FullMethodName       = "/palm.s3.v1.S3/RemoveObject"
)

// S3Client is the client API for S3 service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type S3Client interface {
	CreateBucket(ctx context.Context, in *CreateBucketRequest, opts ...grpc.CallOption) (*CreateBucketResponse, error)
	UploadObject(ctx context.Context, in *UploadObjectRequest, opts ...grpc.CallOption) (*UploadObjectResponse, error)
	ObjectPermanentUrl(ctx context.Context, in *ObjectPermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
	ObjectPresignedUrl(ctx context.Context, in *ObjectPresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error)
	RemoveObject(ctx context.Context, in *RemoveObjectRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
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

func (c *s3Client) UploadObject(ctx context.Context, in *UploadObjectRequest, opts ...grpc.CallOption) (*UploadObjectResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UploadObjectResponse)
	err := c.cc.Invoke(ctx, S3_UploadObject_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) ObjectPermanentUrl(ctx context.Context, in *ObjectPermanentUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_ObjectPermanentUrl_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) ObjectPresignedUrl(ctx context.Context, in *ObjectPresignedUrlRequest, opts ...grpc.CallOption) (*UrlResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UrlResponse)
	err := c.cc.Invoke(ctx, S3_ObjectPresignedUrl_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *s3Client) RemoveObject(ctx context.Context, in *RemoveObjectRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, S3_RemoveObject_FullMethodName, in, out, cOpts...)
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
	UploadObject(context.Context, *UploadObjectRequest) (*UploadObjectResponse, error)
	ObjectPermanentUrl(context.Context, *ObjectPermanentUrlRequest) (*UrlResponse, error)
	ObjectPresignedUrl(context.Context, *ObjectPresignedUrlRequest) (*UrlResponse, error)
	RemoveObject(context.Context, *RemoveObjectRequest) (*emptypb.Empty, error)
	mustEmbedUnimplementedS3Server()
}

// UnimplementedS3Server must be embedded to have forward compatible implementations.
type UnimplementedS3Server struct {
}

func (UnimplementedS3Server) CreateBucket(context.Context, *CreateBucketRequest) (*CreateBucketResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateBucket not implemented")
}
func (UnimplementedS3Server) UploadObject(context.Context, *UploadObjectRequest) (*UploadObjectResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UploadObject not implemented")
}
func (UnimplementedS3Server) ObjectPermanentUrl(context.Context, *ObjectPermanentUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ObjectPermanentUrl not implemented")
}
func (UnimplementedS3Server) ObjectPresignedUrl(context.Context, *ObjectPresignedUrlRequest) (*UrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ObjectPresignedUrl not implemented")
}
func (UnimplementedS3Server) RemoveObject(context.Context, *RemoveObjectRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RemoveObject not implemented")
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

func _S3_UploadObject_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UploadObjectRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).UploadObject(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_UploadObject_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).UploadObject(ctx, req.(*UploadObjectRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_ObjectPermanentUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ObjectPermanentUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).ObjectPermanentUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_ObjectPermanentUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).ObjectPermanentUrl(ctx, req.(*ObjectPermanentUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_ObjectPresignedUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ObjectPresignedUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).ObjectPresignedUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_ObjectPresignedUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).ObjectPresignedUrl(ctx, req.(*ObjectPresignedUrlRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _S3_RemoveObject_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RemoveObjectRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(S3Server).RemoveObject(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: S3_RemoveObject_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(S3Server).RemoveObject(ctx, req.(*RemoveObjectRequest))
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
			MethodName: "UploadObject",
			Handler:    _S3_UploadObject_Handler,
		},
		{
			MethodName: "ObjectPermanentUrl",
			Handler:    _S3_ObjectPermanentUrl_Handler,
		},
		{
			MethodName: "ObjectPresignedUrl",
			Handler:    _S3_ObjectPresignedUrl_Handler,
		},
		{
			MethodName: "RemoveObject",
			Handler:    _S3_RemoveObject_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "s3.proto",
}
