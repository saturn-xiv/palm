package services

import (
	"context"
	"slices"

	"github.com/casbin/casbin/v2"

	v1 "github.com/saturn-xiv/palm/gourd/services/v1"
)

type PolicyHandler struct {
	enforcer *casbin.Enforcer
}

func (p *PolicyHandler) Has(ctx context.Context, user int64, role string) (bool, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return false, err
	}

	roles, err := p.enforcer.GetImplicitRolesForUser(user_c)
	if err != nil {
		return false, err
	}
	{
		role_ := v1.RoleFromName(role)
		role_c, err := role_.Code()
		if err != nil {
			return false, err
		}
		if slices.Contains(roles, role_c) {
			return true, nil
		}
	}
	if role != v1.ROLE_ADMINISTRATOR {
		ok, err := user_.IsAdministrator(p.enforcer)
		if err != nil {
			return false, err
		}
		if ok {
			return true, nil
		}
	}
	return false, nil
}

func (p *PolicyHandler) Can(ctx context.Context, user int64, operation string, resource *v1.Resource) (bool, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return false, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(user_c)
	if err != nil {
		return false, err
	}
	permissions, err := v1.PermissionsFromRules(rules)
	if err != nil {
		return false, err
	}
	for _, it := range permissions {
		if it.Operation == operation {
			if it.Resource.Type == resource.Type {
				if it.Resource.ID == nil {
					return true, nil
				}
				if resource.ID != nil && *resource.ID == *it.Resource.ID {
					return true, nil
				}
			}
		}
	}

	return user_.IsAdministrator(p.enforcer)
}

func (p *PolicyHandler) DeleteUser(ctx context.Context, user int64) error {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return err
	}
	_, err = p.enforcer.DeleteUser(user_c)
	return err
}

func (p *PolicyHandler) DeleteRole(ctx context.Context, role string) error {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return err
	}
	_, err = p.enforcer.DeleteRole(role_c)
	return err
}

func (p *PolicyHandler) GetRolesForUser(ctx context.Context, user int64) ([]string, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetRolesForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []string
	for _, it := range roles {
		role, err := v1.RoleFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, role.Name)
	}
	return items, nil
}

func (p *PolicyHandler) GetImplicitRolesForUser(ctx context.Context, user int64) ([]string, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitRolesForUser(user_c)
	if err != nil {
		return nil, err
	}
	var items []string
	for _, it := range roles {
		role, err := v1.RoleFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, role.Name)
	}
	return items, nil
}

func (p *PolicyHandler) GetUsersForRole(ctx context.Context, role string) ([]int64, error) {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetUsersForRole(role_c)
	if err != nil {
		return nil, err
	}
	var items []int64
	for _, it := range roles {
		user, err := v1.UserFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, user.ID)
	}
	return items, nil
}

func (p *PolicyHandler) GetImplicitUsersForRole(ctx context.Context, role string) ([]int64, error) {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return nil, err
	}

	roles, err := p.enforcer.GetImplicitUsersForRole(role_c)
	if err != nil {
		return nil, err
	}
	var items []int64
	for _, it := range roles {
		user, err := v1.UserFromCode(it)
		if err != nil {
			return nil, err
		}
		items = append(items, user.ID)
	}
	return items, nil
}

func (p *PolicyHandler) AddRolesForUser(ctx context.Context, user int64, roles []string) error {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return err
	}
	var rules []string
	for _, it := range roles {
		role_ := v1.RoleFromName(it)
		role_c, err := role_.Code()
		if err != nil {
			return err
		}
		rules = append(rules, role_c)
	}
	_, err = p.enforcer.AddRolesForUser(user_c, rules)
	return err
}

func (p *PolicyHandler) DeleteRolesForUser(ctx context.Context, user int64, roles []string) error {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return err
	}
	for _, it := range roles {
		role_ := v1.RoleFromName(it)
		role_c, err := role_.Code()
		if err != nil {
			return err
		}
		if _, err = p.enforcer.DeleteRoleForUser(user_c, role_c); err != nil {
			return err
		}
	}

	return nil
}

func (p *PolicyHandler) GetPermissionsForUser(ctx context.Context, user int64) ([]*v1.Permission, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetPermissionsForUser(user_c)
	if err != nil {
		return nil, err
	}
	items, err := v1.PermissionsFromRules(rules)
	if err != nil {
		return nil, err
	}
	return items, nil
}

func (p *PolicyHandler) GetImplicitPermissionsForUser(ctx context.Context, user int64) ([]*v1.Permission, error) {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(user_c)
	if err != nil {
		return nil, err
	}
	items, err := v1.PermissionsFromRules(rules)
	if err != nil {
		return nil, err
	}
	return items, nil
}

func (p *PolicyHandler) AddPermissionsForUser(ctx context.Context, user int64, permissions []*v1.Permission) error {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return err
	}
	rules, err := v1.PermissionsToRules(permissions)
	if err != nil {
		return err
	}
	_, err = p.enforcer.AddPermissionsForUser(user_c, rules...)

	return err

}

func (p *PolicyHandler) DeletePermissionsForUser(ctx context.Context, user int64, permissions []*v1.Permission) error {
	user_ := v1.UserFromId(user)
	user_c, err := user_.Code()
	if err != nil {
		return err
	}
	rules, err := v1.PermissionsToRules(permissions)
	if err != nil {
		return err
	}
	for _, it := range rules {
		if _, err = p.enforcer.DeletePermissionForUser(user_c, it...); err != nil {
			return err
		}
	}

	return nil
}

func (p *PolicyHandler) GetPermissionsForRole(ctx context.Context, role string) ([]*v1.Permission, error) {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetPermissionsForUser(role_c)
	if err != nil {
		return nil, err
	}
	items, err := v1.PermissionsFromRules(rules)
	if err != nil {
		return nil, err
	}
	return items, nil
}

func (p *PolicyHandler) GetImplicitPermissionsForRole(ctx context.Context, role string) ([]*v1.Permission, error) {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return nil, err
	}
	rules, err := p.enforcer.GetImplicitPermissionsForUser(role_c)
	if err != nil {
		return nil, err
	}
	items, err := v1.PermissionsFromRules(rules)
	if err != nil {
		return nil, err
	}
	return items, nil
}

func (p *PolicyHandler) AddPermissionsForRole(ctx context.Context, role string, permissions []*v1.Permission) error {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return err
	}
	rules, err := v1.PermissionsToRules(permissions)
	if err != nil {
		return err
	}
	_, err = p.enforcer.AddPermissionsForUser(role_c, rules...)

	return err

}

func (p *PolicyHandler) DeletePermissionsForRole(ctx context.Context, role string, permissions []*v1.Permission) error {
	role_ := v1.RoleFromName(role)
	role_c, err := role_.Code()
	if err != nil {
		return err
	}
	rules, err := v1.PermissionsToRules(permissions)
	if err != nil {
		return err
	}
	for _, it := range rules {
		if _, err = p.enforcer.DeletePermissionForUser(role_c, it...); err != nil {
			return err
		}
	}

	return nil
}

func NewPolicyHandler(enforcer *casbin.Enforcer) *PolicyHandler {
	return &PolicyHandler{enforcer: enforcer}
}
