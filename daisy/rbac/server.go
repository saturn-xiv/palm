package rbac

import (
	"context"
	"errors"

	"github.com/casbin/casbin/v3"
	"google.golang.org/protobuf/types/known/emptypb"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/crypto"
	v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

type Server struct {
	v2.UnimplementedEnforcerServer

	enforcer *casbin.Enforcer
	db       *gorm.DB
	jwt      *crypto.Jwt
}

func NewServer(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *Server {
	return &Server{enforcer: enforcer, db: db, jwt: jwt}
}

func (p *Server) GetAllSubjects(ctx context.Context, req *emptypb.Empty) (*v2.SubjectsResponse, error) {
	var res v2.SubjectsResponse
	items, err := p.enforcer.GetAllSubjects()
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		sub, err := v2.NewSubject(it)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, sub)
	}
	return &res, nil
}

func (p *Server) GetAllObjects(ctx context.Context, req *emptypb.Empty) (*v2.ObjectsResponse, error) {
	var res v2.ObjectsResponse
	items, err := p.enforcer.GetAllObjects()
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		obj, err := v2.NewObject(it)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, obj)
	}
	return &res, nil
}
func (p *Server) GetAllActions(ctx context.Context, req *emptypb.Empty) (*v2.ActionsResponse, error) {
	var res v2.ActionsResponse
	items, err := p.enforcer.GetAllActions()
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		act, err := v2.NewAction(it)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, act)
	}
	return &res, nil
}
func (p *Server) GetAllRoles(ctx context.Context, req *emptypb.Empty) (*v2.RolesResponse, error) {
	var res v2.RolesResponse
	items, err := p.enforcer.GetAllRoles()
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		sub, err := v2.NewSubject(it)
		if err != nil {
			return nil, err
		}
		switch sub.By.(type) {
		case *v2.Subject_Role_:
			res.Items = append(res.Items, sub.GetRole())
		default:
			return nil, errors.New("not a role subject")
		}

	}
	return &res, nil
}

func (p *Server) GetRolesForUser(ctx context.Context, req *v2.Subject_User) (*v2.RolesResponse, error) {
	var res v2.RolesResponse
	user := v2.Subject{By: &v2.Subject_User_{User: req}}
	sub, err := user.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetRolesForUser(sub)
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		sub, err := v2.NewSubject(it)
		if err != nil {
			return nil, err
		}
		switch sub.By.(type) {
		case *v2.Subject_Role_:
			res.Items = append(res.Items, sub.GetRole())
		default:
			return nil, errors.New("not a role subject")
		}

	}
	return &res, nil
}
func (p *Server) GetImplicitRolesForUser(ctx context.Context, req *v2.Subject_User) (*v2.RolesResponse, error) {
	var res v2.RolesResponse
	user := v2.Subject{By: &v2.Subject_User_{User: req}}
	sub, err := user.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitRolesForUser(sub)
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		sub, err := v2.NewSubject(it)
		if err != nil {
			return nil, err
		}
		switch sub.By.(type) {
		case *v2.Subject_Role_:
			res.Items = append(res.Items, sub.GetRole())
		default:
			return nil, errors.New("not a role subject")
		}

	}
	return &res, nil
}
func (p *Server) GetUsersForRole(ctx context.Context, req *v2.Subject_Role) (*v2.UsersResponse, error) {
	var res v2.UsersResponse
	role := v2.Subject{By: &v2.Subject_Role_{Role: req}}
	sub, err := role.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetUsersForRole(sub)
	if err != nil {
		return nil, err
	}
	for _, it := range items {
		sub, err := v2.NewSubject(it)
		if err != nil {
			return nil, err
		}
		switch sub.By.(type) {
		case *v2.Subject_User_:
			res.Items = append(res.Items, sub.GetUser())
		default:
			return nil, errors.New("not a role subject")
		}

	}
	return &res, nil
}
func (p *Server) HasRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	if err := v2.Has(p.enforcer, req.User, req.Role); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Server) AddRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	role := v2.Subject{By: &v2.Subject_Role_{Role: req.Role}}
	role_s, err := role.ToString()
	if err != nil {
		return nil, err
	}
	user := v2.Subject{By: &v2.Subject_User_{User: req.User}}
	user_s, err := user.ToString()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddRoleForUser(user_s, role_s); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) DeleteRoleForUser(ctx context.Context, req *v2.UserRoleRequest) (*emptypb.Empty, error) {
	role := v2.Subject{By: &v2.Subject_Role_{Role: req.Role}}
	role_s, err := role.ToString()
	if err != nil {
		return nil, err
	}
	user := v2.Subject{By: &v2.Subject_User_{User: req.User}}
	user_s, err := user.ToString()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRoleForUser(user_s, role_s); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) DeleteUser(ctx context.Context, req *v2.Subject_User) (*emptypb.Empty, error) {
	user := v2.Subject{By: &v2.Subject_User_{User: req}}
	user_s, err := user.ToString()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteUser(user_s); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}
func (p *Server) DeleteRole(ctx context.Context, req *v2.Subject_Role) (*emptypb.Empty, error) {
	role := v2.Subject{By: &v2.Subject_Role_{Role: req}}
	role_s, err := role.ToString()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeleteRole(role_s); err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}

func (p *Server) GetPermissionsForUser(ctx context.Context, req *v2.Subject_User) (*v2.PermissionsResponse, error) {
	user := v2.Subject{By: &v2.Subject_User_{User: req}}
	user_s, err := user.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetPermissionsForUser(user_s)
	if err != nil {
		return nil, err
	}
	var res v2.PermissionsResponse
	for _, rules := range items {
		it, err := v2.NewPermission(rules)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, it)
	}
	return &res, nil
}
func (p *Server) GetImplicitPermissionsForUser(ctx context.Context, req *v2.Subject_User) (*v2.PermissionsResponse, error) {
	user := v2.Subject{By: &v2.Subject_User_{User: req}}
	user_s, err := user.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetImplicitPermissionsForUser(user_s)
	if err != nil {
		return nil, err
	}
	var res v2.PermissionsResponse
	for _, rules := range items {
		it, err := v2.NewPermission(rules)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, it)
	}
	return &res, nil
}
func (p *Server) GetPermissionsForRole(ctx context.Context, req *v2.Subject_Role) (*v2.PermissionsResponse, error) {
	role := v2.Subject{By: &v2.Subject_Role_{Role: req}}
	role_s, err := role.ToString()
	if err != nil {
		return nil, err
	}
	items, err := p.enforcer.GetPermissionsForUser(role_s)
	if err != nil {
		return nil, err
	}
	var res v2.PermissionsResponse
	for _, rules := range items {
		it, err := v2.NewPermission(rules)
		if err != nil {
			return nil, err
		}
		res.Items = append(res.Items, it)
	}
	return &res, nil
}
func (p *Server) DeletePermission(ctx context.Context, req *v2.Permission) (*emptypb.Empty, error) {
	rules, err := req.Rules()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.DeletePermission(rules...); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Server) AddPermission(ctx context.Context, req *v2.Permission) (*emptypb.Empty, error) {
	rules, err := req.Rules()
	if err != nil {
		return nil, err
	}
	if _, err = p.enforcer.AddPermissionForUser(rules[0], rules[1], rules[2]); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
func (p *Server) HasPermission(ctx context.Context, req *v2.Permission) (*emptypb.Empty, error) {
	if err := v2.Can(p.enforcer, req.Subject, req.Action, req.Object); err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}
