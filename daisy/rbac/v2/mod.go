package v2

import (
	"encoding/base64"
	"fmt"
	"strings"

	"google.golang.org/protobuf/proto"
)

var (
	ROLE_ADMINISTRATOR = "administrator"
	ROLE_ROOT          = "root"
)

func UserByCode(code string) *Subject {
	return &Subject{
		By: &Subject_User_{
			User: &Subject_User{
				By: &Subject_User_Code{
					Code: code,
				},
			},
		},
	}
}

func UserById(id int64) *Subject {
	return &Subject{
		By: &Subject_User_{
			User: &Subject_User{
				By: &Subject_User_Id{
					Id: id,
				},
			},
		},
	}
}
func RoleAdministrator() *Subject {
	return &Subject{
		By: &Subject_Role_{
			Role: &Subject_Role{
				By: &Subject_Role_Administrator_{
					Administrator: &Subject_Role_Administrator{},
				},
			},
		},
	}
}

func RoleByCode(code string) *Subject {
	return &Subject{
		By: &Subject_Role_{
			Role: &Subject_Role{
				By: &Subject_Role_Code{
					Code: code,
				},
			},
		},
	}
}

func RoleRoot() *Subject {
	return &Subject{
		By: &Subject_Role_{
			Role: &Subject_Role{
				By: &Subject_Role_Root_{
					Root: &Subject_Role_Root{},
				},
			},
		},
	}
}

func NewAction(s string) (*Action, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}

	var it Action
	if err = proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Action) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func NewObject(s string) (*Object, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Object
	if err = proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Object) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func NewSubject(s string) (*Subject, error) {
	tmp, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Subject
	if err = proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Subject) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf), nil
}

func (p *Subject_Role) Root() bool {
	switch p.By.(type) {
	case *Subject_Role_Root_:
		return true
	default:
		return false
	}
}

func (p *Subject_Role) Administrator() bool {
	switch p.By.(type) {
	case *Subject_Role_Administrator_:
		return true
	default:
		return false
	}
}

func (p *Subject_Role) Is(code string) bool {
	switch it := p.By.(type) {
	case *Subject_Role_Code:
		return it.Code == code
	default:
		return false
	}
}

func (p *Action) Inquiry() bool {
	switch p.By.(type) {
	case *Action_Inquiry_:
		return true
	default:
		return false
	}
}
func (p *Action) Debit() bool {
	switch p.By.(type) {
	case *Action_Debit_:
		return true
	default:
		return false
	}
}
func (p *Action) Credit() bool {
	switch p.By.(type) {
	case *Action_Credit_:
		return true
	default:
		return false
	}
}
func (p *Action) Execute() bool {
	switch p.By.(type) {
	case *Action_Execute_:
		return true
	default:
		return false
	}
}
func (p *Action) Append() bool {
	switch p.By.(type) {
	case *Action_Append_:
		return true
	default:
		return false
	}
}
func (p *Action) Write() bool {
	switch p.By.(type) {
	case *Action_Write_:
		return true
	default:
		return false
	}
}
func (p *Action) Read() bool {
	switch p.By.(type) {
	case *Action_Read_:
		return true
	default:
		return false
	}
}
func (p *Action) Is(code string) bool {
	switch it := p.By.(type) {
	case *Action_Code:
		return it.Code == code
	default:
		return false
	}
}

func ActionExecute() *Action {
	return &Action{By: &Action_Execute_{Execute: &Action_Execute{}}}
}

func ActionAppend() *Action {
	return &Action{By: &Action_Append_{Append: &Action_Append{}}}
}
func ActionDelete() *Action {
	return &Action{By: &Action_Code{Code: "delete"}}
}

func ActionManage() *Action {
	return &Action{By: &Action_Code{Code: "manage"}}
}

func NewPermission(rules []string) (*Permission, error) {
	if len(rules) != 3 {
		return nil, fmt.Errorf("unknows rules: %s", strings.Join(rules, ","))
	}
	sub, err := NewSubject(rules[0])
	if err != nil {
		return nil, err
	}
	obj, err := NewObject(rules[1])
	if err != nil {
		return nil, err
	}
	act, err := NewAction(rules[1])
	if err != nil {
		return nil, err
	}
	return &Permission{Subject: sub, Object: obj, Action: act}, nil

}

func (p *Permission) Rules() ([]string, error) {
	sub, err := p.Subject.ToString()
	if err != nil {
		return nil, err
	}
	obj, err := p.Object.ToString()
	if err != nil {
		return nil, err
	}
	act, err := p.Action.ToString()
	if err != nil {
		return nil, err
	}
	return []string{sub, obj, act}, nil
}
