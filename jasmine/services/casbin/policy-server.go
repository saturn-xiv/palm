package casbin

import (
	"context"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type PolicyServer struct {
	v2.UnimplementedPolicyServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *PolicyServer) GetAllUsers(ctx context.Context, req *emptypb.Empty) (*v2.UsersResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetAllObjects(ctx context.Context, req *emptypb.Empty) (*v2.ObjectsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetAllActions(ctx context.Context, req *emptypb.Empty) (*v2.ActionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetAllRoles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}

func (p *PolicyServer) Has(ctx context.Context, req *v2.UserRoleRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) Can(ctx context.Context, req *v2.UserPermissionRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}

func (p *PolicyServer) GetRolesForUser(ctx context.Context, req *v2.User) (*v2.RolesResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetImplicitRolesForUser(ctx context.Context, req *v2.User) (*v2.RolesResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetUsersForRole(ctx context.Context, req *v2.Role) (*v2.UsersResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) HasRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) AddRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) DeleteRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) DeleteUser(ctx context.Context, req *v2.User) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) DeleteRole(ctx context.Context, req *v2.Role) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}

func (p *PolicyServer) GetPermissionsForUser(ctx context.Context, req *v2.User) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetImplicitPermissionsForUser(ctx context.Context, req *v2.User) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) AddPermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) DeletePermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) HasPermissionForUser(ctx context.Context, req *v2.UserPermissionRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}

func (p *PolicyServer) GetPermissionsForRole(ctx context.Context, req *v2.Role) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) GetImplicitPermissionsForRole(ctx context.Context, req *v2.Role) (*v2.PermissionsResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) AddPermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) DeletePermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*emptypb.Empty, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}
func (p *PolicyServer) HasPermissionForRole(ctx context.Context, req *v2.RolePermissionRequest) (*v2.BoolResponse, error) {
	ss := models.SessionFromGrpc(p.db, p.enforcer, p.jwt, ctx)
	if !ss.IsAdministrator() {
		return nil, web.ErrorUserMustHasAdministrator
	}
	// TODO
	return nil, nil
}

func NewPolicyServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyServer {
	return &PolicyServer{db: db, enforcer: enforcer, jwt: jwt}
}
