package services

import (
	"context"
	"errors"
	"slices"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"

	pb "github.com/saturn-xiv/palm/atropa/rbac/services/v2"
)

func NewPolicyService(enforcer *casbin.Enforcer) *PolicyService {
	return &PolicyService{enforcer: enforcer}
}

type PolicyService struct {
	pb.UnimplementedPolicyServer

	enforcer *casbin.Enforcer
}

func (p *PolicyService) Has(ctx context.Context, req *pb.PolicyHasRequest) (*emptypb.Empty, error) {
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}
	role_c, err := req.Role.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitRolesForUser(user_c)
	if err != nil {
		return nil, err
	}

	if slices.Contains(roles, role_c) {
		return &emptypb.Empty{}, nil
	}

	if !req.Role.IsAdministrator() {
		admin := pb.NewPolicyAdministratorRole()
		admin_c, err := admin.Code()
		if err != nil {
			return nil, err
		}
		if slices.Contains(roles, admin_c) {
			return &emptypb.Empty{}, nil
		}
	}

	return nil, errors.New("")
}

func (p *PolicyService) Can(ctx context.Context, req *pb.PolicyCanRequest) (*emptypb.Empty, error) {
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(user_c)
	if err != nil {
		return nil, err
	}

	for _, rule := range rules {
		it, err := pb.NewPolicyPermissionsFromRule(rule)
		if err != nil {
			return nil, err
		}
		if it.Operation == req.Operation {
			if it.Resource.Type == req.Resource.Type {
				if it.Resource.Id == nil {
					return &emptypb.Empty{}, nil
				}
				if it.Resource.Id.Equal(req.Resource.Id) {
					return &emptypb.Empty{}, nil
				}
			}
		}
	}
	return nil, errors.New("")
}

func (p *PolicyService) DeleteUser(ctx context.Context, req *pb.PolicyUsersResponse_Item) (*emptypb.Empty, error) {
	user_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteUser(user_c); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) DeleteRole(ctx context.Context, req *pb.PolicyRolesResponse_Item) (*emptypb.Empty, error) {
	role_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRole(role_c); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) GetRolesForUser(ctx context.Context, req *pb.PolicyUsersResponse_Item) (*pb.PolicyRolesResponse, error) {
	user_c, err := req.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetRolesForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyRolesResponse_Item
	for _, it := range roles {
		role, err := pb.NewPolicyRoleFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, role)
	}
	return &pb.PolicyRolesResponse{Items: items}, nil
}

func (p *PolicyService) GetImplicitRolesForUser(ctx context.Context, req *pb.PolicyUsersResponse_Item) (*pb.PolicyRolesResponse, error) {
	user_c, err := req.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitRolesForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyRolesResponse_Item
	for _, it := range roles {
		role, err := pb.NewPolicyRoleFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, role)
	}
	return &pb.PolicyRolesResponse{Items: items}, nil
}

func (p *PolicyService) GetUsersForRole(ctx context.Context, req *pb.PolicyRolesResponse_Item) (*pb.PolicyUsersResponse, error) {
	role_c, err := req.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetUsersForRole(role_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyUsersResponse_Item
	for _, it := range roles {
		user, err := pb.NewPolicyUserFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, user)
	}
	return &pb.PolicyUsersResponse{Items: items}, nil
}

func (p *PolicyService) GetImplicitUsersForRole(ctx context.Context, req *pb.PolicyRolesResponse_Item) (*pb.PolicyUsersResponse, error) {
	role_c, err := req.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitUsersForRole(role_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyUsersResponse_Item
	for _, it := range roles {
		user, err := pb.NewPolicyUserFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, user)
	}
	return &pb.PolicyUsersResponse{Items: items}, nil
}

func (p *PolicyService) AddRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	if len(req.Roles) == 0 {
		return nil, errors.New("empty roles")
	}
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}
	var rules []string
	for _, it := range req.Roles {
		role_c, err := it.Code()
		if err != nil {
			return nil, err
		}
		rules = append(rules, role_c)
	}

	if _, err = p.enforcer.AddRolesForUser(user_c, rules); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) DeleteRolesForUser(ctx context.Context, req *pb.PolicyRolesForUserRequest) (*emptypb.Empty, error) {
	if len(req.Roles) == 0 {
		return nil, errors.New("empty roles")
	}
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}
	for _, it := range req.Roles {
		role_c, err := it.Code()
		if err != nil {
			return nil, err
		}
		if _, err = p.enforcer.DeleteRoleForUser(user_c, role_c); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) GetPermissionsForUser(ctx context.Context, req *pb.PolicyUsersResponse_Item) (*pb.PolicyPermissionsResponse, error) {
	user_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetPermissionsForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyPermissionsResponse_Item
	for _, rule := range rules {
		permission, err := pb.NewPolicyPermissionsFromRule(rule)
		if err != nil {
			return nil, err
		}
		items = append(items, permission)
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *PolicyService) GetImplicitPermissionsForUser(ctx context.Context, req *pb.PolicyUsersResponse_Item) (*pb.PolicyPermissionsResponse, error) {
	user_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyPermissionsResponse_Item
	for _, rule := range rules {
		permission, err := pb.NewPolicyPermissionsFromRule(rule)
		if err != nil {
			return nil, err
		}
		items = append(items, permission)
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *PolicyService) AddPermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return nil, errors.New("empty permissions")
	}
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}
	var rules [][]string
	for _, it := range req.Permissions {
		rule, err := it.Rule()
		if err != nil {
			return nil, err
		}
		rules = append(rules, rule)
	}
	if _, err = p.enforcer.AddPermissionsForUser(user_c, rules...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) DeletePermissionsForUser(ctx context.Context, req *pb.PolicyPermissionsForUserRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return nil, errors.New("empty permissions")
	}
	user_c, err := req.User.Code()
	if err != nil {
		return nil, err
	}

	for _, it := range req.Permissions {
		rule, err := it.Rule()
		if err != nil {
			return nil, err
		}
		if _, err = p.enforcer.DeletePermissionForUser(user_c, rule...); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) GetPermissionsForRole(ctx context.Context, req *pb.PolicyRolesResponse_Item) (*pb.PolicyPermissionsResponse, error) {
	role_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetPermissionsForUser(role_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyPermissionsResponse_Item
	for _, rule := range rules {
		permission, err := pb.NewPolicyPermissionsFromRule(rule)
		if err != nil {
			return nil, err
		}
		items = append(items, permission)
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *PolicyService) GetImplicitPermissionsForRole(ctx context.Context, req *pb.PolicyRolesResponse_Item) (*pb.PolicyPermissionsResponse, error) {
	role_c, err := req.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(role_c)
	if err != nil {
		return nil, err
	}
	var items []*pb.PolicyPermissionsResponse_Item
	for _, rule := range rules {
		permission, err := pb.NewPolicyPermissionsFromRule(rule)
		if err != nil {
			return nil, err
		}
		items = append(items, permission)
	}
	return &pb.PolicyPermissionsResponse{Items: items}, nil
}

func (p *PolicyService) AddPermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return nil, errors.New("empty permissions")
	}
	role_c, err := req.Role.Code()
	if err != nil {
		return nil, err
	}
	var rules [][]string
	for _, it := range req.Permissions {
		rule, err := it.Rule()
		if err != nil {
			return nil, err
		}
		rules = append(rules, rule)
	}

	if _, err = p.enforcer.AddPermissionsForUser(role_c, rules...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p *PolicyService) DeletePermissionsForRole(ctx context.Context, req *pb.PolicyPermissionsForRoleRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return nil, errors.New("empty permissions")
	}
	role_c, err := req.Role.Code()
	if err != nil {
		return nil, err
	}

	for _, it := range req.Permissions {
		rule, err := it.Rule()
		if err != nil {
			return nil, err
		}
		if _, err = p.enforcer.DeletePermissionForUser(role_c, rule...); err != nil {
			return nil, err
		}
	}
	return &emptypb.Empty{}, nil
}
