package v2

import (
	"encoding/base64"
	"path/filepath"
	"reflect"
	"time"

	"github.com/casbin/casbin/v3"
	"github.com/google/uuid"
	"google.golang.org/protobuf/proto"

	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

func Plugin() string {
	return reflect.TypeOf((*Session)(nil)).Elem().PkgPath()
}

func (p *Session_Subject) ToString() (string, error) {
	buf, err := proto.Marshal(p)
	if err != nil {
		return "", err
	}
	return base64.RawURLEncoding.EncodeToString(buf), nil
}

func NewSubject(s string) (*Session_Subject, error) {
	tmp, err := base64.RawURLEncoding.DecodeString(s)
	if err != nil {
		return nil, err
	}
	var it Session_Subject
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

func (p *UserIndexResponse_Item) HasRole(enforcer *casbin.Enforcer, role string) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Code{
			Code: role,
		},
	})
}

func (p *UserIndexResponse_Item) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Administrator_{
			Administrator: &rbac_v2.Subject_Role_Administrator{},
		},
	})
}

func (p *UserIndexResponse_Item) IsRoot(enforcer *casbin.Enforcer) error {
	return p.Has(enforcer, &rbac_v2.Subject_Role{
		By: &rbac_v2.Subject_Role_Root_{
			Root: &rbac_v2.Subject_Role_Root{},
		},
	})
}

func (p *UserIndexResponse_Item) Has(enforcer *casbin.Enforcer, role *rbac_v2.Subject_Role) error {
	return rbac_v2.Has(enforcer,
		&rbac_v2.Subject_User{
			By: &rbac_v2.Subject_User_Id{
				Id: p.Id,
			},
		}, role)
}

func (p *UserIndexResponse_Item) Can(enforcer *casbin.Enforcer, action *rbac_v2.Action, object *rbac_v2.Object) error {
	return rbac_v2.Can(enforcer, &rbac_v2.Subject{By: &rbac_v2.Subject_User_{User: &rbac_v2.Subject_User{By: &rbac_v2.Subject_User_Id{Id: p.Id}}}}, action, object)
}
