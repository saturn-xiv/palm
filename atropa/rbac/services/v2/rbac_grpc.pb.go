// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.5.1
// - protoc             v5.27.2
// source: rbac.proto

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
// Requires gRPC-Go v1.64.0 or later.
const _ = grpc.SupportPackageIsVersion9

const (
	Policy_Has_FullMethodName                           = "/palm.rbac.v1.Policy/Has"
	Policy_Can_FullMethodName                           = "/palm.rbac.v1.Policy/Can"
	Policy_DeleteUser_FullMethodName                    = "/palm.rbac.v1.Policy/DeleteUser"
	Policy_DeleteRole_FullMethodName                    = "/palm.rbac.v1.Policy/DeleteRole"
	Policy_GetRolesForUser_FullMethodName               = "/palm.rbac.v1.Policy/GetRolesForUser"
	Policy_GetImplicitRolesForUser_FullMethodName       = "/palm.rbac.v1.Policy/GetImplicitRolesForUser"
	Policy_GetUsersForRole_FullMethodName               = "/palm.rbac.v1.Policy/GetUsersForRole"
	Policy_GetImplicitUsersForRole_FullMethodName       = "/palm.rbac.v1.Policy/GetImplicitUsersForRole"
	Policy_AddRolesForUser_FullMethodName               = "/palm.rbac.v1.Policy/AddRolesForUser"
	Policy_DeleteRolesForUser_FullMethodName            = "/palm.rbac.v1.Policy/DeleteRolesForUser"
	Policy_GetPermissionsForUser_FullMethodName         = "/palm.rbac.v1.Policy/GetPermissionsForUser"
	Policy_GetImplicitPermissionsForUser_FullMethodName = "/palm.rbac.v1.Policy/GetImplicitPermissionsForUser"
	Policy_AddPermissionsForUser_FullMethodName         = "/palm.rbac.v1.Policy/AddPermissionsForUser"
	Policy_DeletePermissionsForUser_FullMethodName      = "/palm.rbac.v1.Policy/DeletePermissionsForUser"
	Policy_GetPermissionsForRole_FullMethodName         = "/palm.rbac.v1.Policy/GetPermissionsForRole"
	Policy_GetImplicitPermissionsForRole_FullMethodName = "/palm.rbac.v1.Policy/GetImplicitPermissionsForRole"
	Policy_AddPermissionsForRole_FullMethodName         = "/palm.rbac.v1.Policy/AddPermissionsForRole"
	Policy_DeletePermissionsForRole_FullMethodName      = "/palm.rbac.v1.Policy/DeletePermissionsForRole"
)

// PolicyClient is the client API for Policy service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type PolicyClient interface {
	Has(ctx context.Context, in *PolicyHasRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	Can(ctx context.Context, in *PolicyCanRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	DeleteUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*emptypb.Empty, error)
	DeleteRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*emptypb.Empty, error)
	GetRolesForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyRolesResponse, error)
	GetImplicitRolesForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyRolesResponse, error)
	GetUsersForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyUsersResponse, error)
	GetImplicitUsersForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyUsersResponse, error)
	AddRolesForUser(ctx context.Context, in *PolicyRolesForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	DeleteRolesForUser(ctx context.Context, in *PolicyRolesForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	GetPermissionsForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error)
	GetImplicitPermissionsForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error)
	AddPermissionsForUser(ctx context.Context, in *PolicyPermissionsForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	DeletePermissionsForUser(ctx context.Context, in *PolicyPermissionsForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	GetPermissionsForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error)
	GetImplicitPermissionsForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error)
	AddPermissionsForRole(ctx context.Context, in *PolicyPermissionsForRoleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	DeletePermissionsForRole(ctx context.Context, in *PolicyPermissionsForRoleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
}

type policyClient struct {
	cc grpc.ClientConnInterface
}

func NewPolicyClient(cc grpc.ClientConnInterface) PolicyClient {
	return &policyClient{cc}
}

func (c *policyClient) Has(ctx context.Context, in *PolicyHasRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_Has_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) Can(ctx context.Context, in *PolicyCanRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_Can_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) DeleteUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_DeleteUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) DeleteRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_DeleteRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetRolesForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyRolesResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyRolesResponse)
	err := c.cc.Invoke(ctx, Policy_GetRolesForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetImplicitRolesForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyRolesResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyRolesResponse)
	err := c.cc.Invoke(ctx, Policy_GetImplicitRolesForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetUsersForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyUsersResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyUsersResponse)
	err := c.cc.Invoke(ctx, Policy_GetUsersForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetImplicitUsersForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyUsersResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyUsersResponse)
	err := c.cc.Invoke(ctx, Policy_GetImplicitUsersForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) AddRolesForUser(ctx context.Context, in *PolicyRolesForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_AddRolesForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) DeleteRolesForUser(ctx context.Context, in *PolicyRolesForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_DeleteRolesForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetPermissionsForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyPermissionsResponse)
	err := c.cc.Invoke(ctx, Policy_GetPermissionsForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetImplicitPermissionsForUser(ctx context.Context, in *PolicyUsersResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyPermissionsResponse)
	err := c.cc.Invoke(ctx, Policy_GetImplicitPermissionsForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) AddPermissionsForUser(ctx context.Context, in *PolicyPermissionsForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_AddPermissionsForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) DeletePermissionsForUser(ctx context.Context, in *PolicyPermissionsForUserRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_DeletePermissionsForUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetPermissionsForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyPermissionsResponse)
	err := c.cc.Invoke(ctx, Policy_GetPermissionsForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) GetImplicitPermissionsForRole(ctx context.Context, in *PolicyRolesResponse_Item, opts ...grpc.CallOption) (*PolicyPermissionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(PolicyPermissionsResponse)
	err := c.cc.Invoke(ctx, Policy_GetImplicitPermissionsForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) AddPermissionsForRole(ctx context.Context, in *PolicyPermissionsForRoleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_AddPermissionsForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *policyClient) DeletePermissionsForRole(ctx context.Context, in *PolicyPermissionsForRoleRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, Policy_DeletePermissionsForRole_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// PolicyServer is the server API for Policy service.
// All implementations must embed UnimplementedPolicyServer
// for forward compatibility.
type PolicyServer interface {
	Has(context.Context, *PolicyHasRequest) (*emptypb.Empty, error)
	Can(context.Context, *PolicyCanRequest) (*emptypb.Empty, error)
	DeleteUser(context.Context, *PolicyUsersResponse_Item) (*emptypb.Empty, error)
	DeleteRole(context.Context, *PolicyRolesResponse_Item) (*emptypb.Empty, error)
	GetRolesForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyRolesResponse, error)
	GetImplicitRolesForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyRolesResponse, error)
	GetUsersForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyUsersResponse, error)
	GetImplicitUsersForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyUsersResponse, error)
	AddRolesForUser(context.Context, *PolicyRolesForUserRequest) (*emptypb.Empty, error)
	DeleteRolesForUser(context.Context, *PolicyRolesForUserRequest) (*emptypb.Empty, error)
	GetPermissionsForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyPermissionsResponse, error)
	GetImplicitPermissionsForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyPermissionsResponse, error)
	AddPermissionsForUser(context.Context, *PolicyPermissionsForUserRequest) (*emptypb.Empty, error)
	DeletePermissionsForUser(context.Context, *PolicyPermissionsForUserRequest) (*emptypb.Empty, error)
	GetPermissionsForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyPermissionsResponse, error)
	GetImplicitPermissionsForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyPermissionsResponse, error)
	AddPermissionsForRole(context.Context, *PolicyPermissionsForRoleRequest) (*emptypb.Empty, error)
	DeletePermissionsForRole(context.Context, *PolicyPermissionsForRoleRequest) (*emptypb.Empty, error)
	mustEmbedUnimplementedPolicyServer()
}

// UnimplementedPolicyServer must be embedded to have
// forward compatible implementations.
//
// NOTE: this should be embedded by value instead of pointer to avoid a nil
// pointer dereference when methods are called.
type UnimplementedPolicyServer struct{}

func (UnimplementedPolicyServer) Has(context.Context, *PolicyHasRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Has not implemented")
}
func (UnimplementedPolicyServer) Can(context.Context, *PolicyCanRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Can not implemented")
}
func (UnimplementedPolicyServer) DeleteUser(context.Context, *PolicyUsersResponse_Item) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteUser not implemented")
}
func (UnimplementedPolicyServer) DeleteRole(context.Context, *PolicyRolesResponse_Item) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteRole not implemented")
}
func (UnimplementedPolicyServer) GetRolesForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyRolesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetRolesForUser not implemented")
}
func (UnimplementedPolicyServer) GetImplicitRolesForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyRolesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetImplicitRolesForUser not implemented")
}
func (UnimplementedPolicyServer) GetUsersForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyUsersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetUsersForRole not implemented")
}
func (UnimplementedPolicyServer) GetImplicitUsersForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyUsersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetImplicitUsersForRole not implemented")
}
func (UnimplementedPolicyServer) AddRolesForUser(context.Context, *PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddRolesForUser not implemented")
}
func (UnimplementedPolicyServer) DeleteRolesForUser(context.Context, *PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteRolesForUser not implemented")
}
func (UnimplementedPolicyServer) GetPermissionsForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyPermissionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPermissionsForUser not implemented")
}
func (UnimplementedPolicyServer) GetImplicitPermissionsForUser(context.Context, *PolicyUsersResponse_Item) (*PolicyPermissionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetImplicitPermissionsForUser not implemented")
}
func (UnimplementedPolicyServer) AddPermissionsForUser(context.Context, *PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddPermissionsForUser not implemented")
}
func (UnimplementedPolicyServer) DeletePermissionsForUser(context.Context, *PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeletePermissionsForUser not implemented")
}
func (UnimplementedPolicyServer) GetPermissionsForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyPermissionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPermissionsForRole not implemented")
}
func (UnimplementedPolicyServer) GetImplicitPermissionsForRole(context.Context, *PolicyRolesResponse_Item) (*PolicyPermissionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetImplicitPermissionsForRole not implemented")
}
func (UnimplementedPolicyServer) AddPermissionsForRole(context.Context, *PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddPermissionsForRole not implemented")
}
func (UnimplementedPolicyServer) DeletePermissionsForRole(context.Context, *PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeletePermissionsForRole not implemented")
}
func (UnimplementedPolicyServer) mustEmbedUnimplementedPolicyServer() {}
func (UnimplementedPolicyServer) testEmbeddedByValue()                {}

// UnsafePolicyServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to PolicyServer will
// result in compilation errors.
type UnsafePolicyServer interface {
	mustEmbedUnimplementedPolicyServer()
}

func RegisterPolicyServer(s grpc.ServiceRegistrar, srv PolicyServer) {
	// If the following call pancis, it indicates UnimplementedPolicyServer was
	// embedded by pointer and is nil.  This will cause panics if an
	// unimplemented method is ever invoked, so we test this at initialization
	// time to prevent it from happening at runtime later due to I/O.
	if t, ok := srv.(interface{ testEmbeddedByValue() }); ok {
		t.testEmbeddedByValue()
	}
	s.RegisterService(&Policy_ServiceDesc, srv)
}

func _Policy_Has_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyHasRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).Has(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_Has_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).Has(ctx, req.(*PolicyHasRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_Can_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyCanRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).Can(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_Can_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).Can(ctx, req.(*PolicyCanRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_DeleteUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyUsersResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).DeleteUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_DeleteUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).DeleteUser(ctx, req.(*PolicyUsersResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_DeleteRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).DeleteRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_DeleteRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).DeleteRole(ctx, req.(*PolicyRolesResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetRolesForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyUsersResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetRolesForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetRolesForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetRolesForUser(ctx, req.(*PolicyUsersResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetImplicitRolesForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyUsersResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetImplicitRolesForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetImplicitRolesForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetImplicitRolesForUser(ctx, req.(*PolicyUsersResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetUsersForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetUsersForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetUsersForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetUsersForRole(ctx, req.(*PolicyRolesResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetImplicitUsersForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetImplicitUsersForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetImplicitUsersForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetImplicitUsersForRole(ctx, req.(*PolicyRolesResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_AddRolesForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesForUserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).AddRolesForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_AddRolesForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).AddRolesForUser(ctx, req.(*PolicyRolesForUserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_DeleteRolesForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesForUserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).DeleteRolesForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_DeleteRolesForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).DeleteRolesForUser(ctx, req.(*PolicyRolesForUserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetPermissionsForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyUsersResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetPermissionsForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetPermissionsForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetPermissionsForUser(ctx, req.(*PolicyUsersResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetImplicitPermissionsForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyUsersResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetImplicitPermissionsForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetImplicitPermissionsForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetImplicitPermissionsForUser(ctx, req.(*PolicyUsersResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_AddPermissionsForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyPermissionsForUserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).AddPermissionsForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_AddPermissionsForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).AddPermissionsForUser(ctx, req.(*PolicyPermissionsForUserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_DeletePermissionsForUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyPermissionsForUserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).DeletePermissionsForUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_DeletePermissionsForUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).DeletePermissionsForUser(ctx, req.(*PolicyPermissionsForUserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetPermissionsForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetPermissionsForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetPermissionsForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetPermissionsForRole(ctx, req.(*PolicyRolesResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_GetImplicitPermissionsForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyRolesResponse_Item)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).GetImplicitPermissionsForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_GetImplicitPermissionsForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).GetImplicitPermissionsForRole(ctx, req.(*PolicyRolesResponse_Item))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_AddPermissionsForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyPermissionsForRoleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).AddPermissionsForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_AddPermissionsForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).AddPermissionsForRole(ctx, req.(*PolicyPermissionsForRoleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Policy_DeletePermissionsForRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PolicyPermissionsForRoleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(PolicyServer).DeletePermissionsForRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Policy_DeletePermissionsForRole_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(PolicyServer).DeletePermissionsForRole(ctx, req.(*PolicyPermissionsForRoleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Policy_ServiceDesc is the grpc.ServiceDesc for Policy service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Policy_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "palm.rbac.v1.Policy",
	HandlerType: (*PolicyServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Has",
			Handler:    _Policy_Has_Handler,
		},
		{
			MethodName: "Can",
			Handler:    _Policy_Can_Handler,
		},
		{
			MethodName: "DeleteUser",
			Handler:    _Policy_DeleteUser_Handler,
		},
		{
			MethodName: "DeleteRole",
			Handler:    _Policy_DeleteRole_Handler,
		},
		{
			MethodName: "GetRolesForUser",
			Handler:    _Policy_GetRolesForUser_Handler,
		},
		{
			MethodName: "GetImplicitRolesForUser",
			Handler:    _Policy_GetImplicitRolesForUser_Handler,
		},
		{
			MethodName: "GetUsersForRole",
			Handler:    _Policy_GetUsersForRole_Handler,
		},
		{
			MethodName: "GetImplicitUsersForRole",
			Handler:    _Policy_GetImplicitUsersForRole_Handler,
		},
		{
			MethodName: "AddRolesForUser",
			Handler:    _Policy_AddRolesForUser_Handler,
		},
		{
			MethodName: "DeleteRolesForUser",
			Handler:    _Policy_DeleteRolesForUser_Handler,
		},
		{
			MethodName: "GetPermissionsForUser",
			Handler:    _Policy_GetPermissionsForUser_Handler,
		},
		{
			MethodName: "GetImplicitPermissionsForUser",
			Handler:    _Policy_GetImplicitPermissionsForUser_Handler,
		},
		{
			MethodName: "AddPermissionsForUser",
			Handler:    _Policy_AddPermissionsForUser_Handler,
		},
		{
			MethodName: "DeletePermissionsForUser",
			Handler:    _Policy_DeletePermissionsForUser_Handler,
		},
		{
			MethodName: "GetPermissionsForRole",
			Handler:    _Policy_GetPermissionsForRole_Handler,
		},
		{
			MethodName: "GetImplicitPermissionsForRole",
			Handler:    _Policy_GetImplicitPermissionsForRole_Handler,
		},
		{
			MethodName: "AddPermissionsForRole",
			Handler:    _Policy_AddPermissionsForRole_Handler,
		},
		{
			MethodName: "DeletePermissionsForRole",
			Handler:    _Policy_DeletePermissionsForRole_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "rbac.proto",
}
