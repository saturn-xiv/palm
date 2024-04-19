package v2

import (
	"encoding/base64"
	"fmt"
	"log/slog"

	"google.golang.org/protobuf/proto"
)

func (p *Role) Display() string {
	switch it := p.By.(type) {
	case *Role_Administrator_:
		return "Administrator"
	case *Role_Root_:
		return "Root"
	case *Role_Member:
		return it.Member
	default:
		return "Unknown"
	}
}

func (p *Resource) Display() string {
	if p.Id == nil {
		return fmt.Sprintf("%s://*", p.Type)
	}
	return fmt.Sprintf("%s://%d", p.Type, *p.Id)
}

func (p *User) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func (p *Role) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func (p *Resource) Code() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}
func ResourceFromCode(code string) (*Resource, error) {
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it Resource
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func RoleFromCode(code string) (*Role, error) {
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it Role
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func UserFromCode(code string) (*User, error) {
	buf, err := base64.StdEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	var it User
	if err = proto.Unmarshal(buf, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func FromPermissions(permissions [][]string) ([]*Permission, error) {
	var items []*Permission
	for _, pm := range permissions {
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

func ToPermissions(permissions []*Permission) ([][]string, error) {
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
