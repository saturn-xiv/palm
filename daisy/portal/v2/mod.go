package v2

import (
	"encoding/base64"
	"errors"
	"path/filepath"
	"reflect"
	"slices"
	"time"

	"github.com/casbin/casbin/v3"
	"github.com/google/uuid"
	"google.golang.org/protobuf/proto"

	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

func Plugin() string {
	return reflect.TypeOf((*Session)(nil)).Elem().PkgPath()
}

func (p *Session) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(buf), nil
}

func NewSession(s string) (*Session, error) {
	tmp, err := base64.URLEncoding.DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Session
	if err := proto.Unmarshal(tmp, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Page) Offset() int64 {
	return (p.Index - 1) * p.Size
}

func NewPagination(page *Page, total int64) *Pagination {
	size := page.Size
	if size < 20 {
		size = 20
	}
	if size > 1000 {
		size = 1000
	}
	index := page.Index
	if index < 1 {
		index = 1
	}
	pages := total / size
	if total%size > 0 {
		pages = pages + 1
	}
	if index*size > total {
		index = pages
	}
	return &Pagination{
		Current:     &Page{Index: index, Size: size},
		Total:       total,
		Pages:       pages,
		HasPrevious: index > 1,
		HasNext:     index < pages,
	}
}

func (p *UserCreateAttachmentRequest) Bucket() string {
	return "attachments-" + time.Now().Format(time.DateOnly)
}
func (p *UserCreateAttachmentRequest) Object() string {
	return uuid.New().String() + filepath.Ext(p.Title)
}

func (p *Session) HasRole(enforcer *casbin.Enforcer, role string) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Code{
			Code: role,
		},
	})
}

func (p *Session) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Administrator_{
			Administrator: &rbac_v2.Subject_Role_Administrator{},
		},
	})
}

func (p *Session) IsRoot(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Root_{
			Root: &rbac_v2.Subject_Role_Root{},
		},
	})
}

func (p *Session) Has(enforcer *casbin.Enforcer, role *rbac_v2.Subject_Role) error {
	return has_role(enforcer,
		&rbac_v2.Subject_User{
			By: &rbac_v2.Subject_User_Id{
				Id: p.User.Id,
			},
		}, role)
}

func has_role(enforcer *casbin.Enforcer, user *rbac_v2.Subject_User, role *rbac_v2.Subject_Role) error {
	role_ := rbac_v2.Subject{By: &rbac_v2.Subject_Role_{Role: role}}
	role_s, err := role_.ToString()
	if err != nil {
		return err
	}

	user_ := rbac_v2.Subject{By: &rbac_v2.Subject_User_{User: user}}
	user_s, err := user_.ToString()
	if err != nil {
		return err
	}

	items, err := enforcer.GetImplicitRolesForUser(user_s)
	if err != nil {
		return err
	}
	if slices.Contains(items, role_s) {
		return nil
	}
	return errors.New("deny")
}

func (p *Session) Can(enforcer *casbin.Enforcer, action *rbac_v2.Action, object *rbac_v2.Object) error {
	return can(enforcer, &rbac_v2.Subject{By: &rbac_v2.Subject_User_{User: &rbac_v2.Subject_User{By: &rbac_v2.Subject_User_Id{Id: p.User.Id}}}}, action, object)
}

func can(enforcer *casbin.Enforcer, subject *rbac_v2.Subject, action *rbac_v2.Action, object *rbac_v2.Object) error {
	subject_s, err := subject.ToString()
	if err != nil {
		return err
	}
	object_s, err := object.ToString()
	if err != nil {
		return err
	}
	action_s, err := action.ToString()
	if err != nil {
		return err
	}
	items, err := enforcer.GetImplicitPermissionsForUser(subject_s)
	if err != nil {
		return err
	}
	for _, it := range items {
		if len(it) == 4 {
			if it[0] == "p" && it[1] == subject_s && it[2] == object_s && it[3] == action_s {
				return nil
			}
		}
	}
	return errors.New("deny")
}
