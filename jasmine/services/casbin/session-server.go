package casbin

import (
	"context"
	"slices"

	"github.com/casbin/casbin/v2"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type SessionServer struct {
	v2.UnimplementedSessionServer

	jwt      *crypto.Jwt
	db       *gorm.DB
	enforcer *casbin.Enforcer
}

func (p *SessionServer) Has(ctx context.Context, req *v2.Role) (*v2.BoolResponse, error) {
	user, err := p.current_user(ctx)
	if err != nil {
		return nil, err
	}
	role, err := web.ProtoBufMessageToString(v2.NewRoleSubject(req))
	if err != nil {
		return nil, err
	}
	roles, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	reply.Yes = slices.Contains(roles, role)
	return &reply, nil
}

func (p *SessionServer) Can(ctx context.Context, req *v2.SessionCanRequest) (*v2.BoolResponse, error) {
	user, err := p.current_user(ctx)
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
	permissions, err := p.enforcer.GetImplicitPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	var reply v2.BoolResponse
	for _, rule := range permissions {
		if len(rule) != 3 {
			return nil, v2.ErrorUnknownPermissionRule(rule)
		}
		if rule[1] == obj && rule[2] == act {
			reply.Yes = true
			break
		}
	}
	return &reply, nil
}

func (p *SessionServer) Roles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	user, err := p.current_user(ctx)
	if err != nil {
		return nil, err
	}
	roles, err := p.enforcer.GetRolesForUser(user)
	if err != nil {
		return nil, err
	}
	return new_roles_response(roles)
}

func (p *SessionServer) Permissions(ctx context.Context, req *emptypb.Empty) (*v2.PermissionsResponse, error) {
	user, err := p.current_user(ctx)
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(rules)
}

func (p *SessionServer) ImplicitRoles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	user, err := p.current_user(ctx)
	if err != nil {
		return nil, err
	}
	roles, err := p.enforcer.GetImplicitRolesForUser(user)
	if err != nil {
		return nil, err
	}
	return new_roles_response(roles)
}

func (p *SessionServer) ImplicitPermissions(ctx context.Context, req *emptypb.Empty) (*v2.PermissionsResponse, error) {
	user, err := p.current_user(ctx)
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(user)
	if err != nil {
		return nil, err
	}
	return new_permissions_response(rules)
}

func NewSessionServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *SessionServer {
	return &SessionServer{db: db, enforcer: enforcer, jwt: jwt}
}

func (p *SessionServer) current_user(ctx context.Context) (string, error) {
	ss := web.SessionFromGrpc(ctx)
	user, err := models.CurrentUser(p.db, p.jwt, ss)
	if err != nil {
		return "", err
	}
	return web.ProtoBufMessageToString(v2.NewUserSubjectById(user.ID))
}

func new_roles_response(roles []string) (*v2.RolesResponse, error) {
	var reply v2.RolesResponse
	for _, r := range roles {
		var it v2.Subject
		if err := web.ProtoBufMessageFromString(r, &it); err != nil {
			return nil, err
		}
		role := it.GetRole()
		if role == nil {
			continue
		}
		reply.Items = append(reply.Items, role)
	}
	return &reply, nil
}

func new_permissions_response(rules [][]string) (*v2.PermissionsResponse, error) {
	var reply v2.PermissionsResponse
	for _, rule := range rules {
		if len(rule) != 3 {
			return nil, v2.ErrorUnknownPermissionRule(rule)
		}
		var sub v2.Subject
		if err := web.ProtoBufMessageFromString(rule[0], &sub); err != nil {
			return nil, err
		}
		var obj v2.Object
		if err := web.ProtoBufMessageFromString(rule[1], &obj); err != nil {
			return nil, err
		}
		var act v2.Action
		if err := web.ProtoBufMessageFromString(rule[2], &act); err != nil {
			return nil, err
		}
		reply.Items = append(reply.Items, &v2.Permission{Action: &act, Object: &obj, Subject: &sub})
	}
	return &reply, nil
}
