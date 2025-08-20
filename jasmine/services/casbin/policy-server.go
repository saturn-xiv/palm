package casbin

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	portal_v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type PolicyServer struct {
	v2.UnimplementedPolicyServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *PolicyServer) GetAllUsers(ctx context.Context, req *emptypb.Empty) (*v2.UsersResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetAllSubjects()
	if err != nil {
		return nil, err
	}
	var reply v2.UsersResponse
	for _, it := range items {
		var sub v2.Subject
		if err = web.ProtoBufMessageFromString(it, &sub); err != nil {
			return nil, err
		}
		user := sub.GetUser()
		if user == nil {
			continue
		}
		reply.Items = append(reply.Items, user)
	}
	return &reply, nil
}
func (p *PolicyServer) GetAllObjects(ctx context.Context, req *emptypb.Empty) (*v2.ObjectsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetAllObjects()
	if err != nil {
		return nil, err
	}
	var reply v2.ObjectsResponse
	for _, it := range items {
		var obj v2.Object
		if err = web.ProtoBufMessageFromString(it, &obj); err != nil {
			reply.Items = append(reply.Items, &obj)
		}
	}
	return &reply, nil
}
func (p *PolicyServer) GetAllActions(ctx context.Context, req *emptypb.Empty) (*v2.ActionsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetAllActions()
	if err != nil {
		return nil, err
	}
	var reply v2.ActionsResponse
	for _, it := range items {
		var act v2.Action
		if err = web.ProtoBufMessageFromString(it, &act); err != nil {
			reply.Items = append(reply.Items, &act)
		}
	}
	return &reply, nil
}
func (p *PolicyServer) GetAllRoles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetAllRoles()
	if err != nil {
		return nil, err
	}
	return new_roles_response(items)
}

func (p *PolicyServer) Has(ctx context.Context, req *v2.UserRoleRequest) (*v2.BoolResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	reply.Yes = v2.Has(items, req.Role)
	return &reply, nil
}
func (p *PolicyServer) Can(ctx context.Context, req *v2.UserPermissionRequest) (*v2.BoolResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	reply.Yes = v2.Can(items, req.Object, req.Action)
	return &reply, nil
}

func (p *PolicyServer) GetRolesForUser(ctx context.Context, req *v2.User) (*v2.RolesResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetRolesForUser(user)
	if err != nil {
		return nil, err
	}
	return new_roles_response(items)
}
func (p *PolicyServer) GetImplicitRolesForUser(ctx context.Context, req *v2.User) (*v2.RolesResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	return new_roles_response(items)
}
func (p *PolicyServer) GetUsersForRole(ctx context.Context, req *v2.Role) (*v2.UsersResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetUsersForRole(role)
	if err != nil {
		return nil, err
	}
	var reply v2.UsersResponse
	for _, r := range items {
		var it v2.Subject
		if err := web.ProtoBufMessageFromString(r, &it); err != nil {
			return nil, err
		}
		user := it.GetUser()
		if user == nil {
			continue
		}
		reply.Items = append(reply.Items, user)
	}
	return &reply, nil
}
func (p *PolicyServer) HasRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*v2.BoolResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	if reply.Yes, err = p.enforcer.HasRoleForUser(user, role); err != nil {
		return nil, err
	}
	return &reply, nil
}
func (p *PolicyServer) AddRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddRoleForUser(user, role); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) DeleteRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRoleForUser(user, role); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) DeleteUser(ctx context.Context, req *v2.User) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	user, err := web.ProtoBufMessageToString(v2.NewUserSubject(req))
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteUser(user); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) DeleteRole(ctx context.Context, req *v2.Role) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}

	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req))
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRole(role); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}

func (p *PolicyServer) GetPermissionsForUser(ctx context.Context, req *v2.User) (*v2.PermissionsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewUserSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetPermissionsForUser(sub)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(items)
}
func (p *PolicyServer) GetImplicitPermissionsForUser(ctx context.Context, req *v2.User) (*v2.PermissionsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewUserSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitPermissionsForUser(sub)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(items)
}
func (p *PolicyServer) AddPermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}

	if _, err := p.enforcer.AddPermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) DeletePermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}

	if _, err := p.enforcer.DeletePermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) HasPermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*v2.BoolResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewUserSubject(req.User))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	if reply.Yes, err = p.enforcer.HasPermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &reply, nil
}

func (p *PolicyServer) GetPermissionsForRole(ctx context.Context, req *v2.Role) (*v2.PermissionsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetPermissionsForUser(sub)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(items)
}
func (p *PolicyServer) GetImplicitPermissionsForRole(ctx context.Context, req *v2.Role) (*v2.PermissionsResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req))
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitPermissionsForUser(sub)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(items)
}
func (p *PolicyServer) AddPermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}

	if _, err := p.enforcer.AddPermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) DeletePermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*emptypb.Empty, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}

	if _, err := p.enforcer.DeletePermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *PolicyServer) HasPermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*v2.BoolResponse, error) {
	_, err := p.must_administrator(ctx)
	if err != nil {
		return nil, err
	}
	sub, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req.Role))
	if err != nil {
		return nil, err
	}
	obj, err := web.ProtoBufMessageToString(req.Object)
	if err != nil {
		return nil, err
	}
	act, err := web.ProtoBufMessageToString(req.Action)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	if reply.Yes, err = p.enforcer.HasPermissionForUser(sub, obj, act); err != nil {
		return nil, err
	}
	return &reply, nil
}

func NewPolicyServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyServer {
	return &PolicyServer{db: db, enforcer: enforcer, jwt: jwt}
}
func (p *PolicyServer) must_administrator(ctx context.Context) (*models.Session, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, portal_v2.ErrorUserMustHasAdministrator
	}
	return ss, nil
}
