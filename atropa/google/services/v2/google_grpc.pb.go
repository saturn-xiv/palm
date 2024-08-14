// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v4.25.3
// source: google.proto

package v2

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.62.0 or later.
const _ = grpc.SupportPackageIsVersion8

const (
	Oauth2_AuthCodeURL_FullMethodName = "/palm.google.v1.Oauth2/AuthCodeURL"
	Oauth2_SignIn_FullMethodName      = "/palm.google.v1.Oauth2/SignIn"
)

// Oauth2Client is the client API for Oauth2 service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type Oauth2Client interface {
	AuthCodeURL(ctx context.Context, in *Oauth2AuthCodeURLRequest, opts ...grpc.CallOption) (*Oauth2AuthCodeURLResponse, error)
	SignIn(ctx context.Context, in *Oauth2SignInRequest, opts ...grpc.CallOption) (*Oauth2SignInResponse, error)
}

type oauth2Client struct {
	cc grpc.ClientConnInterface
}

func NewOauth2Client(cc grpc.ClientConnInterface) Oauth2Client {
	return &oauth2Client{cc}
}

func (c *oauth2Client) AuthCodeURL(ctx context.Context, in *Oauth2AuthCodeURLRequest, opts ...grpc.CallOption) (*Oauth2AuthCodeURLResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(Oauth2AuthCodeURLResponse)
	err := c.cc.Invoke(ctx, Oauth2_AuthCodeURL_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *oauth2Client) SignIn(ctx context.Context, in *Oauth2SignInRequest, opts ...grpc.CallOption) (*Oauth2SignInResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(Oauth2SignInResponse)
	err := c.cc.Invoke(ctx, Oauth2_SignIn_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// Oauth2Server is the server API for Oauth2 service.
// All implementations must embed UnimplementedOauth2Server
// for forward compatibility
type Oauth2Server interface {
	AuthCodeURL(context.Context, *Oauth2AuthCodeURLRequest) (*Oauth2AuthCodeURLResponse, error)
	SignIn(context.Context, *Oauth2SignInRequest) (*Oauth2SignInResponse, error)
	mustEmbedUnimplementedOauth2Server()
}

// UnimplementedOauth2Server must be embedded to have forward compatible implementations.
type UnimplementedOauth2Server struct {
}

func (UnimplementedOauth2Server) AuthCodeURL(context.Context, *Oauth2AuthCodeURLRequest) (*Oauth2AuthCodeURLResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AuthCodeURL not implemented")
}
func (UnimplementedOauth2Server) SignIn(context.Context, *Oauth2SignInRequest) (*Oauth2SignInResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SignIn not implemented")
}
func (UnimplementedOauth2Server) mustEmbedUnimplementedOauth2Server() {}

// UnsafeOauth2Server may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to Oauth2Server will
// result in compilation errors.
type UnsafeOauth2Server interface {
	mustEmbedUnimplementedOauth2Server()
}

func RegisterOauth2Server(s grpc.ServiceRegistrar, srv Oauth2Server) {
	s.RegisterService(&Oauth2_ServiceDesc, srv)
}

func _Oauth2_AuthCodeURL_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Oauth2AuthCodeURLRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(Oauth2Server).AuthCodeURL(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Oauth2_AuthCodeURL_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(Oauth2Server).AuthCodeURL(ctx, req.(*Oauth2AuthCodeURLRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Oauth2_SignIn_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Oauth2SignInRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(Oauth2Server).SignIn(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Oauth2_SignIn_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(Oauth2Server).SignIn(ctx, req.(*Oauth2SignInRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Oauth2_ServiceDesc is the grpc.ServiceDesc for Oauth2 service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Oauth2_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.google.v1.Oauth2",
	HandlerType: (*Oauth2Server)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "AuthCodeURL",
			Handler:    _Oauth2_AuthCodeURL_Handler,
		},
		{
			MethodName: "SignIn",
			Handler:    _Oauth2_SignIn_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "google.proto",
}
