// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v3.21.12
// source: balsam.proto

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
	Aes_Encrypt_FullMethodName = "/palm.balsam.v1.Aes/Encrypt"
	Aes_Decrypt_FullMethodName = "/palm.balsam.v1.Aes/Decrypt"
)

// AesClient is the client API for Aes service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type AesClient interface {
	Encrypt(ctx context.Context, in *AesPlainMessage, opts ...grpc.CallOption) (*AesCodeMessage, error)
	Decrypt(ctx context.Context, in *AesCodeMessage, opts ...grpc.CallOption) (*AesPlainMessage, error)
}

type aesClient struct {
	cc grpc.ClientConnInterface
}

func NewAesClient(cc grpc.ClientConnInterface) AesClient {
	return &aesClient{cc}
}

func (c *aesClient) Encrypt(ctx context.Context, in *AesPlainMessage, opts ...grpc.CallOption) (*AesCodeMessage, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AesCodeMessage)
	err := c.cc.Invoke(ctx, Aes_Encrypt_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *aesClient) Decrypt(ctx context.Context, in *AesCodeMessage, opts ...grpc.CallOption) (*AesPlainMessage, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AesPlainMessage)
	err := c.cc.Invoke(ctx, Aes_Decrypt_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// AesServer is the server API for Aes service.
// All implementations must embed UnimplementedAesServer
// for forward compatibility
type AesServer interface {
	Encrypt(context.Context, *AesPlainMessage) (*AesCodeMessage, error)
	Decrypt(context.Context, *AesCodeMessage) (*AesPlainMessage, error)
	mustEmbedUnimplementedAesServer()
}

// UnimplementedAesServer must be embedded to have forward compatible implementations.
type UnimplementedAesServer struct {
}

func (UnimplementedAesServer) Encrypt(context.Context, *AesPlainMessage) (*AesCodeMessage, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Encrypt not implemented")
}
func (UnimplementedAesServer) Decrypt(context.Context, *AesCodeMessage) (*AesPlainMessage, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Decrypt not implemented")
}
func (UnimplementedAesServer) mustEmbedUnimplementedAesServer() {}

// UnsafeAesServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to AesServer will
// result in compilation errors.
type UnsafeAesServer interface {
	mustEmbedUnimplementedAesServer()
}

func RegisterAesServer(s grpc.ServiceRegistrar, srv AesServer) {
	s.RegisterService(&Aes_ServiceDesc, srv)
}

func _Aes_Encrypt_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AesPlainMessage)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AesServer).Encrypt(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Aes_Encrypt_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AesServer).Encrypt(ctx, req.(*AesPlainMessage))
	}
	return interceptor(ctx, in, info, handler)
}

func _Aes_Decrypt_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AesCodeMessage)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AesServer).Decrypt(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Aes_Decrypt_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AesServer).Decrypt(ctx, req.(*AesCodeMessage))
	}
	return interceptor(ctx, in, info, handler)
}

// Aes_ServiceDesc is the grpc.ServiceDesc for Aes service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Aes_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.balsam.v1.Aes",
	HandlerType: (*AesServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Encrypt",
			Handler:    _Aes_Encrypt_Handler,
		},
		{
			MethodName: "Decrypt",
			Handler:    _Aes_Decrypt_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "balsam.proto",
}

const (
	Jwt_Sign_FullMethodName   = "/palm.balsam.v1.Jwt/Sign"
	Jwt_Verify_FullMethodName = "/palm.balsam.v1.Jwt/Verify"
)

// JwtClient is the client API for Jwt service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type JwtClient interface {
	Sign(ctx context.Context, in *JwtSignRequest, opts ...grpc.CallOption) (*JwtSignResponse, error)
	Verify(ctx context.Context, in *JwtVerifyRequest, opts ...grpc.CallOption) (*JwtVerifyResponse, error)
}

type jwtClient struct {
	cc grpc.ClientConnInterface
}

func NewJwtClient(cc grpc.ClientConnInterface) JwtClient {
	return &jwtClient{cc}
}

func (c *jwtClient) Sign(ctx context.Context, in *JwtSignRequest, opts ...grpc.CallOption) (*JwtSignResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(JwtSignResponse)
	err := c.cc.Invoke(ctx, Jwt_Sign_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *jwtClient) Verify(ctx context.Context, in *JwtVerifyRequest, opts ...grpc.CallOption) (*JwtVerifyResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(JwtVerifyResponse)
	err := c.cc.Invoke(ctx, Jwt_Verify_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// JwtServer is the server API for Jwt service.
// All implementations must embed UnimplementedJwtServer
// for forward compatibility
type JwtServer interface {
	Sign(context.Context, *JwtSignRequest) (*JwtSignResponse, error)
	Verify(context.Context, *JwtVerifyRequest) (*JwtVerifyResponse, error)
	mustEmbedUnimplementedJwtServer()
}

// UnimplementedJwtServer must be embedded to have forward compatible implementations.
type UnimplementedJwtServer struct {
}

func (UnimplementedJwtServer) Sign(context.Context, *JwtSignRequest) (*JwtSignResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Sign not implemented")
}
func (UnimplementedJwtServer) Verify(context.Context, *JwtVerifyRequest) (*JwtVerifyResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Verify not implemented")
}
func (UnimplementedJwtServer) mustEmbedUnimplementedJwtServer() {}

// UnsafeJwtServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to JwtServer will
// result in compilation errors.
type UnsafeJwtServer interface {
	mustEmbedUnimplementedJwtServer()
}

func RegisterJwtServer(s grpc.ServiceRegistrar, srv JwtServer) {
	s.RegisterService(&Jwt_ServiceDesc, srv)
}

func _Jwt_Sign_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(JwtSignRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(JwtServer).Sign(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Jwt_Sign_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(JwtServer).Sign(ctx, req.(*JwtSignRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Jwt_Verify_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(JwtVerifyRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(JwtServer).Verify(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Jwt_Verify_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(JwtServer).Verify(ctx, req.(*JwtVerifyRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Jwt_ServiceDesc is the grpc.ServiceDesc for Jwt service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Jwt_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.balsam.v1.Jwt",
	HandlerType: (*JwtServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Sign",
			Handler:    _Jwt_Sign_Handler,
		},
		{
			MethodName: "Verify",
			Handler:    _Jwt_Verify_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "balsam.proto",
}

const (
	HMac_Sign_FullMethodName   = "/palm.balsam.v1.HMac/Sign"
	HMac_Verify_FullMethodName = "/palm.balsam.v1.HMac/Verify"
)

// HMacClient is the client API for HMac service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type HMacClient interface {
	Sign(ctx context.Context, in *HMacSignRequest, opts ...grpc.CallOption) (*HMacSignResponse, error)
	Verify(ctx context.Context, in *HMacVerifyRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
}

type hMacClient struct {
	cc grpc.ClientConnInterface
}

func NewHMacClient(cc grpc.ClientConnInterface) HMacClient {
	return &hMacClient{cc}
}

func (c *hMacClient) Sign(ctx context.Context, in *HMacSignRequest, opts ...grpc.CallOption) (*HMacSignResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(HMacSignResponse)
	err := c.cc.Invoke(ctx, HMac_Sign_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *hMacClient) Verify(ctx context.Context, in *HMacVerifyRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, HMac_Verify_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// HMacServer is the server API for HMac service.
// All implementations must embed UnimplementedHMacServer
// for forward compatibility
type HMacServer interface {
	Sign(context.Context, *HMacSignRequest) (*HMacSignResponse, error)
	Verify(context.Context, *HMacVerifyRequest) (*emptypb.Empty, error)
	mustEmbedUnimplementedHMacServer()
}

// UnimplementedHMacServer must be embedded to have forward compatible implementations.
type UnimplementedHMacServer struct {
}

func (UnimplementedHMacServer) Sign(context.Context, *HMacSignRequest) (*HMacSignResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Sign not implemented")
}
func (UnimplementedHMacServer) Verify(context.Context, *HMacVerifyRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Verify not implemented")
}
func (UnimplementedHMacServer) mustEmbedUnimplementedHMacServer() {}

// UnsafeHMacServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to HMacServer will
// result in compilation errors.
type UnsafeHMacServer interface {
	mustEmbedUnimplementedHMacServer()
}

func RegisterHMacServer(s grpc.ServiceRegistrar, srv HMacServer) {
	s.RegisterService(&HMac_ServiceDesc, srv)
}

func _HMac_Sign_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(HMacSignRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(HMacServer).Sign(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: HMac_Sign_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(HMacServer).Sign(ctx, req.(*HMacSignRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _HMac_Verify_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(HMacVerifyRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(HMacServer).Verify(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: HMac_Verify_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(HMacServer).Verify(ctx, req.(*HMacVerifyRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// HMac_ServiceDesc is the grpc.ServiceDesc for HMac service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var HMac_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.balsam.v1.HMac",
	HandlerType: (*HMacServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Sign",
			Handler:    _HMac_Sign_Handler,
		},
		{
			MethodName: "Verify",
			Handler:    _HMac_Verify_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "balsam.proto",
}

const (
	User_SignInByEmail_FullMethodName = "/palm.balsam.v1.User/SignInByEmail"
)

// UserClient is the client API for User service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type UserClient interface {
	SignInByEmail(ctx context.Context, in *UserSignInByEmail, opts ...grpc.CallOption) (*UserSignInResponse, error)
}

type userClient struct {
	cc grpc.ClientConnInterface
}

func NewUserClient(cc grpc.ClientConnInterface) UserClient {
	return &userClient{cc}
}

func (c *userClient) SignInByEmail(ctx context.Context, in *UserSignInByEmail, opts ...grpc.CallOption) (*UserSignInResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UserSignInResponse)
	err := c.cc.Invoke(ctx, User_SignInByEmail_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// UserServer is the server API for User service.
// All implementations must embed UnimplementedUserServer
// for forward compatibility
type UserServer interface {
	SignInByEmail(context.Context, *UserSignInByEmail) (*UserSignInResponse, error)
	mustEmbedUnimplementedUserServer()
}

// UnimplementedUserServer must be embedded to have forward compatible implementations.
type UnimplementedUserServer struct {
}

func (UnimplementedUserServer) SignInByEmail(context.Context, *UserSignInByEmail) (*UserSignInResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SignInByEmail not implemented")
}
func (UnimplementedUserServer) mustEmbedUnimplementedUserServer() {}

// UnsafeUserServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to UserServer will
// result in compilation errors.
type UnsafeUserServer interface {
	mustEmbedUnimplementedUserServer()
}

func RegisterUserServer(s grpc.ServiceRegistrar, srv UserServer) {
	s.RegisterService(&User_ServiceDesc, srv)
}

func _User_SignInByEmail_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserSignInByEmail)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(UserServer).SignInByEmail(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: User_SignInByEmail_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(UserServer).SignInByEmail(ctx, req.(*UserSignInByEmail))
	}
	return interceptor(ctx, in, info, handler)
}

// User_ServiceDesc is the grpc.ServiceDesc for User service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var User_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.balsam.v1.User",
	HandlerType: (*UserServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "SignInByEmail",
			Handler:    _User_SignInByEmail_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "balsam.proto",
}
