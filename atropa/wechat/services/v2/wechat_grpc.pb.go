// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v3.21.12
// source: wechat.proto

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
	Oauth2_QrConnectUrl_FullMethodName = "/palm.wechat.v1.Oauth2/QrConnectUrl"
	Oauth2_SignIn_FullMethodName       = "/palm.wechat.v1.Oauth2/SignIn"
)

// Oauth2Client is the client API for Oauth2 service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type Oauth2Client interface {
	QrConnectUrl(ctx context.Context, in *Oauth2QrConnectUrlRequest, opts ...grpc.CallOption) (*Oauth2QrConnectUrlResponse, error)
	SignIn(ctx context.Context, in *Oauth2SignInRequest, opts ...grpc.CallOption) (*Oauth2SignInResponse, error)
}

type oauth2Client struct {
	cc grpc.ClientConnInterface
}

func NewOauth2Client(cc grpc.ClientConnInterface) Oauth2Client {
	return &oauth2Client{cc}
}

func (c *oauth2Client) QrConnectUrl(ctx context.Context, in *Oauth2QrConnectUrlRequest, opts ...grpc.CallOption) (*Oauth2QrConnectUrlResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(Oauth2QrConnectUrlResponse)
	err := c.cc.Invoke(ctx, Oauth2_QrConnectUrl_FullMethodName, in, out, cOpts...)
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
	QrConnectUrl(context.Context, *Oauth2QrConnectUrlRequest) (*Oauth2QrConnectUrlResponse, error)
	SignIn(context.Context, *Oauth2SignInRequest) (*Oauth2SignInResponse, error)
	mustEmbedUnimplementedOauth2Server()
}

// UnimplementedOauth2Server must be embedded to have forward compatible implementations.
type UnimplementedOauth2Server struct {
}

func (UnimplementedOauth2Server) QrConnectUrl(context.Context, *Oauth2QrConnectUrlRequest) (*Oauth2QrConnectUrlResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method QrConnectUrl not implemented")
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

func _Oauth2_QrConnectUrl_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Oauth2QrConnectUrlRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(Oauth2Server).QrConnectUrl(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Oauth2_QrConnectUrl_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(Oauth2Server).QrConnectUrl(ctx, req.(*Oauth2QrConnectUrlRequest))
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
	ServiceName: "palm.wechat.v1.Oauth2",
	HandlerType: (*Oauth2Server)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "QrConnectUrl",
			Handler:    _Oauth2_QrConnectUrl_Handler,
		},
		{
			MethodName: "SignIn",
			Handler:    _Oauth2_SignIn_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "wechat.proto",
}

const (
	MiniProgram_Code2Session_FullMethodName = "/palm.wechat.v1.MiniProgram/Code2Session"
)

// MiniProgramClient is the client API for MiniProgram service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MiniProgramClient interface {
	Code2Session(ctx context.Context, in *MiniProgramCode2SessionRequest, opts ...grpc.CallOption) (*MiniProgramCode2SessionResponse, error)
}

type miniProgramClient struct {
	cc grpc.ClientConnInterface
}

func NewMiniProgramClient(cc grpc.ClientConnInterface) MiniProgramClient {
	return &miniProgramClient{cc}
}

func (c *miniProgramClient) Code2Session(ctx context.Context, in *MiniProgramCode2SessionRequest, opts ...grpc.CallOption) (*MiniProgramCode2SessionResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(MiniProgramCode2SessionResponse)
	err := c.cc.Invoke(ctx, MiniProgram_Code2Session_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MiniProgramServer is the server API for MiniProgram service.
// All implementations must embed UnimplementedMiniProgramServer
// for forward compatibility
type MiniProgramServer interface {
	Code2Session(context.Context, *MiniProgramCode2SessionRequest) (*MiniProgramCode2SessionResponse, error)
	mustEmbedUnimplementedMiniProgramServer()
}

// UnimplementedMiniProgramServer must be embedded to have forward compatible implementations.
type UnimplementedMiniProgramServer struct {
}

func (UnimplementedMiniProgramServer) Code2Session(context.Context, *MiniProgramCode2SessionRequest) (*MiniProgramCode2SessionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Code2Session not implemented")
}
func (UnimplementedMiniProgramServer) mustEmbedUnimplementedMiniProgramServer() {}

// UnsafeMiniProgramServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MiniProgramServer will
// result in compilation errors.
type UnsafeMiniProgramServer interface {
	mustEmbedUnimplementedMiniProgramServer()
}

func RegisterMiniProgramServer(s grpc.ServiceRegistrar, srv MiniProgramServer) {
	s.RegisterService(&MiniProgram_ServiceDesc, srv)
}

func _MiniProgram_Code2Session_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MiniProgramCode2SessionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MiniProgramServer).Code2Session(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: MiniProgram_Code2Session_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MiniProgramServer).Code2Session(ctx, req.(*MiniProgramCode2SessionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// MiniProgram_ServiceDesc is the grpc.ServiceDesc for MiniProgram service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var MiniProgram_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.wechat.v1.MiniProgram",
	HandlerType: (*MiniProgramServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Code2Session",
			Handler:    _MiniProgram_Code2Session_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "wechat.proto",
}

// PayClient is the client API for Pay service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
//
// ----------------------------------------------------------------------------
type PayClient interface {
}

type payClient struct {
	cc grpc.ClientConnInterface
}

func NewPayClient(cc grpc.ClientConnInterface) PayClient {
	return &payClient{cc}
}

// PayServer is the server API for Pay service.
// All implementations must embed UnimplementedPayServer
// for forward compatibility
//
// ----------------------------------------------------------------------------
type PayServer interface {
	mustEmbedUnimplementedPayServer()
}

// UnimplementedPayServer must be embedded to have forward compatible implementations.
type UnimplementedPayServer struct {
}

func (UnimplementedPayServer) mustEmbedUnimplementedPayServer() {}

// UnsafePayServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to PayServer will
// result in compilation errors.
type UnsafePayServer interface {
	mustEmbedUnimplementedPayServer()
}

func RegisterPayServer(s grpc.ServiceRegistrar, srv PayServer) {
	s.RegisterService(&Pay_ServiceDesc, srv)
}

// Pay_ServiceDesc is the grpc.ServiceDesc for Pay service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Pay_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.wechat.v1.Pay",
	HandlerType: (*PayServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams:     []grpc.StreamDesc{},
	Metadata:    "wechat.proto",
}
