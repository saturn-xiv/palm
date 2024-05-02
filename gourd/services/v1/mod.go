package v1

import (
	"bytes"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	"log/slog"
	"strings"

	"github.com/casbin/casbin/v2"
)

func RoleFromName(name string) *Role {
	return &Role{Name: strings.ToLower(strings.TrimSpace(name))}
}

func UserFromId(id int64) *User {
	return &User{ID: id}
}

func (p *Resource) Display() string {
	if p.ID == nil {
		return fmt.Sprintf("%s://*", p.Type)
	}
	return fmt.Sprintf("%s://%d", p.Type, *p.ID)
}

func (p *User) Code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}

func (p *User) IsAdministrator(enforcer *casbin.Enforcer) (bool, error) {
	return p.has_role(enforcer, ROLE_ADMINISTRATOR)
}

func (p *User) IsRoot(enforcer *casbin.Enforcer) (bool, error) {
	return p.has_role(enforcer, ROLE_ROOT)
}

func (p *User) has_role(enforcer *casbin.Enforcer, name string) (bool, error) {
	role := Role{Name: name}
	role_c, err := role.Code()
	if err != nil {
		return false, err
	}
	user_c, err := p.Code()
	if err != nil {
		return false, err
	}
	ok, err := enforcer.HasRoleForUser(user_c, role_c)
	if err != nil {
		return false, err
	}

	return ok, nil
}

func (p *Role) Code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}
func (p *Role) IsAdministrator() bool {
	return p.Name == ROLE_ADMINISTRATOR
}
func (p *Role) IsRoot() bool {
	return p.Name == ROLE_ROOT
}

func (p *Resource) Code() (string, error) {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(p); err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf.Bytes()), nil
}
func ResourceFromCode(code string) (*Resource, error) {
	bin, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	var it Resource
	dec := gob.NewDecoder(buf)
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func RoleFromCode(code string) (*Role, error) {
	bin, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	var it Role
	dec := gob.NewDecoder(buf)
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func UserFromCode(code string) (*User, error) {
	bin, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	buf := bytes.NewBuffer(bin)
	var it User
	dec := gob.NewDecoder(buf)
	if err = dec.Decode(&it); err != nil {
		return nil, err
	}
	return &it, nil
}

func PermissionsFromRules(rules [][]string) ([]*Permission, error) {
	var items []*Permission
	for _, pm := range rules {
		if len(pm) != 3 {
			slog.Error(fmt.Sprintf("unexpected permission %v", pm))
			continue
		}
		var it Permission
		it.Operation = pm[2]
		resource, err := ResourceFromCode(pm[1])
		if err != nil {
			return nil, err
		}
		it.Resource = resource
		items = append(items, &it)
	}
	return items, nil
}

func PermissionsToRules(permissions []*Permission) ([][]string, error) {
	var items [][]string
	for _, pm := range permissions {
		resource, err := pm.Resource.Code()
		if err != nil {
			return nil, err
		}
		items = append(items, []string{resource, pm.Operation})
	}
	return items, nil
}
