package models

import (
	"fmt"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	rbac_pb "github.com/saturn-xiv/palm/lilac/rbac/v2"
)

type User struct {
	Model `gorm:"embedded"`

	Lang              string `gorm:"index:,not null;size:15"`
	Timezone          string `gorm:"index:,not null;size:31"`
	SignInCount       uint32 `gorm:"not null"`
	CurrentSignedInAt *time.Time
	CurrentSignedInIp *string `gorm:"size:45"`
	LastSignedInAt    *time.Time
	LastSignedInIp    *string `gorm:"size:45"`
	LockedAt          *time.Time
}

func (p *User) IsRoot(enforcer *casbin.Enforcer) error {
	return p.HasRole(enforcer, &rbac_pb.Role{
		By: &rbac_pb.Role_Root_{},
	})
}

func (p *User) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.HasRole(enforcer, &rbac_pb.Role{
		By: &rbac_pb.Role_Administrator_{},
	})
}
func (p *User) Has(enforcer *casbin.Enforcer, role string) error {
	return p.HasRole(enforcer, &rbac_pb.Role{
		By: &rbac_pb.Role_Member{
			Member: role,
		},
	})
}

func (p *User) HasRole(enforcer *casbin.Enforcer, role *rbac_pb.Role) error {
	user := rbac_pb.User{Id: p.ID}
	user_c, err := user.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	role_c, err := role.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	ok, err := enforcer.HasRoleForUser(user_c, role_c)
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	if !ok {
		return status.Errorf(codes.PermissionDenied, "user didn't have role of %s", role.Display())
	}
	return nil
}
func (p *User) Can(enforcer *casbin.Enforcer, operation string, resource_type string) error {
	return p.HasPermission(enforcer, &rbac_pb.Permission{
		Operation: operation,
		Resource: &rbac_pb.Resource{
			Type: resource_type,
		},
	})
}

func (p *User) Can_(enforcer *casbin.Enforcer, operation string, resource_type string, resource_id uint32) error {
	return p.HasPermission(enforcer, &rbac_pb.Permission{
		Operation: operation,
		Resource: &rbac_pb.Resource{
			Type: resource_type,
			Id:   &resource_id,
		},
	})
}

func (p *User) HasPermission(enforcer *casbin.Enforcer, permission *rbac_pb.Permission) error {
	user := rbac_pb.User{Id: p.ID}
	subject, err := user.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	object, err := permission.Resource.Code()
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}

	items, err := enforcer.GetImplicitPermissionsForUser(subject)
	if err != nil {
		return status.Errorf(codes.Internal, "%v", err)
	}
	for _, it := range items {
		if len(it) != 3 {
			continue
		}
		if it[1] == object && it[2] == permission.Operation {
			return nil
		}
	}
	return status.Errorf(codes.PermissionDenied, "user didn't have permission(%s, %s)", permission.Operation, permission.Resource.Display())
}

func NewUserDetail(db *gorm.DB, id uint32) (*rbac_pb.UserDetail, error) {
	it := rbac_pb.UserDetail{Id: id}
	{
		var eu EmailUser
		if rst := db.Where(&EmailUser{UserID: id}).First(&eu); rst.Error == nil {
			it.Provider = &rbac_pb.UserDetail_Provider{
				Type: rbac_pb.UserDetail_Provider_Email,
				Id:   eu.Nickname,
			}
			it.RealName = eu.RealName
			return &it, nil
		}
	}
	{
		var gu GoogleUser
		if rst := db.Where(&GoogleUser{UserID: id}).First(&gu); rst.Error == nil {
			it.Provider = &rbac_pb.UserDetail_Provider{
				Type: rbac_pb.UserDetail_Provider_Google,
				Id:   gu.Sub,
			}
			it.RealName = gu.Name
			return &it, nil
		}
	}
	{
		var wu WechatOauth2User
		if rst := db.Where(&WechatOauth2User{UserID: id}).First(&wu); rst.Error == nil {
			it.Provider = &rbac_pb.UserDetail_Provider{
				Type: rbac_pb.UserDetail_Provider_WeChatOauth,
				Id:   wu.OpenId,
			}
			it.RealName = wu.Nickname
			return &it, nil
		}
	}
	{
		var wu WechatMiniProgramUser
		if rst := db.Where(&WechatMiniProgramUser{UserID: id}).First(&wu); rst.Error == nil {
			it.Provider = &rbac_pb.UserDetail_Provider{
				Type: rbac_pb.UserDetail_Provider_WeChatMiniProgram,
				Id:   wu.OpenId,
			}
			if wu.Nickname != nil {
				it.RealName = *wu.Nickname
			}
			return &it, nil
		}
	}
	return nil, fmt.Errorf("couldn't find user details for %d", id)
}
