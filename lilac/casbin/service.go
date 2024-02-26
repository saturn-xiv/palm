package casbin

import (
	"context"
	"errors"
	"slices"
	"strings"

	"github.com/casbin/casbin/v2"
	log "github.com/sirupsen/logrus"
	"google.golang.org/protobuf/types/known/emptypb"

	pb "github.com/saturn-xiv/palm/lilac/casbin/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	enforcer *casbin.Enforcer
}

func NewPolicyService(enforcer *casbin.Enforcer) PolicyService {
	return PolicyService{enforcer: enforcer}
}

func (p PolicyService) Can(_ctx context.Context, req *pb.CanRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}
	object, err := req.Object.ToObject()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetImplicitPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	for _, it := range permissions {
		if len(it) >= 3 {
			if it[1] == object && it[2] == req.Action {
				return &emptypb.Empty{}, nil
			}
		}
	}
	return nil, errors.New("not matched")
}

func (p PolicyService) Has(_ctx context.Context, req *pb.HasRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}
	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	if slices.Contains(roles, role) {
		return &emptypb.Empty{}, nil
	}
	return nil, errors.New("not matched")
}

func (p PolicyService) AddRoleForUser(_ctx context.Context, req *pb.RoleForUserRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}
	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}

	log.Infof("add role(%s) for user(%s)", role, user)
	if _, err = p.enforcer.AddRoleForUser(user, role); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil

}
func (p PolicyService) GetImplicitRolesForUser(_ctx context.Context, req *pb.User) (*pb.RolesResponse, error) {
	user, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Role, 0)
	roles, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	for _, role := range roles {
		it, err := pb.NewRole(role)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.RolesResponse{Items: items}, nil
}

func (p PolicyService) SetRolesForUser(_ctx context.Context, req *pb.RolesForUserRequest) (*emptypb.Empty, error) {
	if len(req.Roles) == 0 {
		return &emptypb.Empty{}, nil
	}
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}
	roles := make([]string, 0)
	for _, it := range req.Roles {
		role, err := it.ToSubject()
		if err != nil {
			return nil, err
		}
		roles = append(roles, role)
	}
	log.Infof("delete roles for user(%s)", user)
	if _, err = p.enforcer.DeleteRolesForUser(user); err != nil {
		return nil, err
	}

	log.Infof("add roles(%s) for user(%s)", strings.Join(roles, ","), user)
	if _, err = p.enforcer.AddRolesForUser(user, roles); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil

}
func (p PolicyService) GetRolesForUser(_ctx context.Context, req *pb.User) (*pb.RolesResponse, error) {
	user, err := req.ToSubject()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetRolesForUser(user)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Role, 0)
	for _, role := range roles {
		it, err := pb.NewRole(role)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.RolesResponse{Items: items}, nil
}
func (p PolicyService) DeleteUser(_ctx context.Context, req *pb.User) (*emptypb.Empty, error) {
	user, err := req.ToSubject()
	if err != nil {
		return nil, err
	}

	log.Infof("delete user(%s)", user)
	if _, err := p.enforcer.DeleteUser(user); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (p PolicyService) GetUsersForRole(_ctx context.Context, req *pb.Role) (*pb.UsersResponse, error) {
	role, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	users, err := p.enforcer.GetUsersForRole(role)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.User, 0)
	for _, user := range users {
		it, err := pb.NewUser(user)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.UsersResponse{Items: items}, nil
}
func (p PolicyService) GetImplicitUsersForRole(_ctx context.Context, req *pb.Role) (*pb.UsersResponse, error) {
	role, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	users, err := p.enforcer.GetImplicitUsersForRole(role)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.User, 0)
	for _, user := range users {
		it, err := pb.NewUser(user)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.UsersResponse{Items: items}, nil
}
func (p PolicyService) DeleteRole(_ctx context.Context, req *pb.Role) (*emptypb.Empty, error) {
	role, err := req.ToSubject()
	if err != nil {
		return nil, err
	}

	log.Infof("delete role(%s)", role)
	if _, err = p.enforcer.DeleteRole(role); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) DeleteRoleForUser(_ctx context.Context, req *pb.RoleForUserRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}

	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}

	log.Infof("delete role(%s) for user(%s)", role, user)
	if _, err = p.enforcer.DeleteRoleForUser(user, role); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) AddPermissionForUser(_ctx context.Context, req *pb.PermissionForUserRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}

	object, err := req.Permission.Resource.ToObject()
	if err != nil {
		return nil, err
	}
	log.Debugf("add permission for user(%s): action(%s) object(%s)", user, req.Permission.Action, object)
	if _, err = p.enforcer.AddPermissionForUser(user, object, req.Permission.Action); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) AddPermissionsForUser(_ctx context.Context, req *pb.PermissionsForUserRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return &emptypb.Empty{}, nil
	}
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}

	permissions := make([][]string, 0)
	for _, it := range req.Permissions {
		object, err := it.Resource.ToObject()
		if err != nil {
			return nil, err
		}
		permissions = append(permissions, []string{object, it.Action})
	}
	if _, err = p.enforcer.AddPermissionsForUser(user, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) GetPermissionsForUser(_ctx context.Context, req *pb.User) (*pb.PermissionsResponse, error) {
	user, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Permission, 0)
	for _, permission := range permissions {
		it, err := pb.NewPermission(permission...)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PermissionsResponse{Items: items}, nil
}
func (p PolicyService) GetImplicitPermissionsForUser(_ctx context.Context, req *pb.User) (*pb.PermissionsResponse, error) {
	user, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetImplicitPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Permission, 0)
	for _, permission := range permissions {
		it, err := pb.NewPermission(permission...)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PermissionsResponse{Items: items}, nil
}
func (p PolicyService) DeletePermissionForUser(_ctx context.Context, req *pb.PermissionForUserRequest) (*emptypb.Empty, error) {
	user, err := req.User.ToSubject()
	if err != nil {
		return nil, err
	}
	object, err := req.Permission.Resource.ToObject()
	if err != nil {
		return nil, err
	}
	log.Debugf("delete permission for user(%s): action(%s) object(%s)", user, req.Permission.Action, object)
	if _, err = p.enforcer.DeletePermissionForUser(user, object, req.Permission.Action); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) AddPermissionForRole(_ctx context.Context, req *pb.PermissionForRoleRequest) (*emptypb.Empty, error) {
	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}

	object, err := req.Permission.Resource.ToObject()
	if err != nil {
		return nil, err
	}

	log.Debugf("add permission for role(%s): action(%s) object(%s)", role, req.Permission.Action, object)
	if _, err = p.enforcer.AddPermissionForUser(role, object, req.Permission.Action); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) AddPermissionsForRole(_ctx context.Context, req *pb.PermissionsForRoleRequest) (*emptypb.Empty, error) {
	if len(req.Permissions) == 0 {
		return &emptypb.Empty{}, nil
	}
	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}

	permissions := make([][]string, 0)
	for _, it := range req.Permissions {
		object, err := it.Resource.ToObject()
		if err != nil {
			return nil, err
		}
		permissions = append(permissions, []string{object, it.Action})
	}
	if _, err = p.enforcer.AddPermissionsForUser(role, permissions...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p PolicyService) GetPermissionsForRole(_ctx context.Context, req *pb.Role) (*pb.PermissionsResponse, error) {
	role, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetPermissionsForUser(role)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Permission, 0)
	for _, permission := range permissions {
		it, err := pb.NewPermission(permission...)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PermissionsResponse{Items: items}, nil
}
func (p PolicyService) GetImplicitPermissionsForRole(_ctx context.Context, req *pb.Role) (*pb.PermissionsResponse, error) {
	role, err := req.ToSubject()
	if err != nil {
		return nil, err
	}
	permissions, err := p.enforcer.GetPermissionsForUser(role)
	if err != nil {
		return nil, err
	}
	items := make([]*pb.Permission, 0)
	for _, permission := range permissions {
		it, err := pb.NewPermission(permission...)
		if err != nil {
			return nil, err
		}
		items = append(items, it)
	}
	return &pb.PermissionsResponse{Items: items}, nil
}
func (p PolicyService) DeletePermissionForRole(_ctx context.Context, req *pb.PermissionForRoleRequest) (*emptypb.Empty, error) {
	role, err := req.Role.ToSubject()
	if err != nil {
		return nil, err
	}
	object, err := req.Permission.Resource.ToObject()
	if err != nil {
		return nil, err
	}
	log.Debugf("delete permission for role(%s): action(%s) object(%s)", role, req.Permission.Action, object)
	if _, err = p.enforcer.DeletePermissionForUser(role, object, req.Permission.Action); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
