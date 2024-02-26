package casbin

import (
	"context"

	casbin_ "github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"

	pb "github.com/saturn-xiv/palm/lilac/casbin/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	enforcer *casbin_.Enforcer
}

func NewPolicyService(enforcer *casbin_.Enforcer) PolicyService {
	return PolicyService{enforcer: enforcer}
}

func (p PolicyService) Can(_ctx context.Context, _req *pb.CanRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p PolicyService) Has(_ctx context.Context, _req *pb.HasRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p PolicyService) AddRolesForUser(_ctx context.Context, _req *pb.RolesForUserRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}
func (p PolicyService) GetImplicitRolesForUser(_ctx context.Context, _req *pb.User) (*pb.RolesResponse, error) {
	// TODO
	return &pb.RolesResponse{}, nil
}
func (p PolicyService) GetRolesForUser(_ctx context.Context, _req *pb.User) (*pb.RolesResponse, error) {
	// TODO
	return &pb.RolesResponse{}, nil
}
func (p PolicyService) DeleteUser(_ctx context.Context, _req *pb.User) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p PolicyService) GetUsersForRole(_ctx context.Context, _req *pb.Role) (*pb.UsersResponse, error) {
	// TODO
	return &pb.UsersResponse{}, nil
}
func (p PolicyService) GetImplicitUsersForRole(_ctx context.Context, _req *pb.Role) (*pb.UsersResponse, error) {
	// TODO
	return &pb.UsersResponse{}, nil
}
func (p PolicyService) DeleteRole(_ctx context.Context, _req *pb.Role) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}
func (p PolicyService) DeleteRolesForUser(_ctx context.Context, _req *pb.RolesForUserRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p PolicyService) AddPermissionsForUser(_ctx context.Context, _req *pb.PermissionsForUserRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}
func (p PolicyService) GetPermissionsForUser(_ctx context.Context, _req *pb.User) (*pb.PermissionsResponse, error) {
	// TODO
	return &pb.PermissionsResponse{}, nil
}
func (p PolicyService) GetImplicitPermissionsForUser(_ctx context.Context, _req *pb.User) (*pb.PermissionsResponse, error) {
	// TODO
	return &pb.PermissionsResponse{}, nil
}
func (p PolicyService) DeletePermissionsForUser(_ctx context.Context, _req *pb.PermissionsForUserRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}

func (p PolicyService) AddPermissionsForRole(_ctx context.Context, _req *pb.PermissionsForRoleRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}
func (p PolicyService) GetPermissionsForRole(_ctx context.Context, _req *pb.Role) (*pb.PermissionsResponse, error) {
	// TODO
	return &pb.PermissionsResponse{}, nil
}
func (p PolicyService) GetImplicitPermissionsForRole(_ctx context.Context, _req *pb.Role) (*pb.PermissionsResponse, error) {
	// TODO
	return &pb.PermissionsResponse{}, nil
}
func (p PolicyService) DeletePermissionsForRole(_ctx context.Context, _req *pb.PermissionsForRoleRequest) (*emptypb.Empty, error) {
	// TODO
	return &emptypb.Empty{}, nil
}
