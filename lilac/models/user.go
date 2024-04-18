package models

import (
	"fmt"
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/services/v2"
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
	return p.HasRole(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Root_{},
	})
}

func (p *User) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.HasRole(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Administrator_{},
	})
}
func (p *User) Has(enforcer *casbin.Enforcer, role string) error {
	return p.HasRole(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
			Member: role,
		},
	})
}

func (p *User) HasRole(enforcer *casbin.Enforcer, role *pb.PolicyRolesResponse_Item) error {
	user := pb.PolicyUserRequest{Id: p.ID}
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
	return p.HasPermission(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
		},
	})
}

func (p *User) Can_(enforcer *casbin.Enforcer, operation string, resource_type string, resource_id uint32) error {
	return p.HasPermission(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
			Id:   &resource_id,
		},
	})
}

func (p *User) HasPermission(enforcer *casbin.Enforcer, permission *pb.PolicyPermissionsResponse_Item) error {
	user := pb.PolicyUserRequest{Id: p.ID}
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

func NewPolicyUserItem(db *gorm.DB, id uint32) (*pb.PolicyUsersResponse_Item, error) {
	it := pb.PolicyUsersResponse_Item{Id: id}
	{
		var eu EmailUser
		if rst := db.Where(&EmailUser{UserID: id}).First(&eu); rst.Error == nil {
			it.ProviderType = pb.UserIndexResponse_Item_Email
			it.ProviderId = eu.Nickname
			it.RealName = eu.RealName
			return &it, nil
		}
	}
	{
		var gu GoogleUser
		if rst := db.Where(&GoogleUser{UserID: id}).First(&gu); rst.Error == nil {
			it.ProviderType = pb.UserIndexResponse_Item_Google
			it.ProviderId = gu.Sub
			it.RealName = gu.Name
			return &it, nil
		}
	}
	{
		var wu WechatOauth2User
		if rst := db.Where(&WechatOauth2User{UserID: id}).First(&wu); rst.Error == nil {
			it.ProviderType = pb.UserIndexResponse_Item_WeChatOauth
			it.ProviderId = wu.OpenId
			it.RealName = wu.Nickname
			return &it, nil
		}
	}
	{
		var wu WechatMiniProgramUser
		if rst := db.Where(&WechatMiniProgramUser{UserID: id}).First(&wu); rst.Error == nil {
			it.ProviderType = pb.UserIndexResponse_Item_WeChatMiniProgram
			it.ProviderId = wu.OpenId
			if wu.Nickname != nil {
				it.RealName = *wu.Nickname
			}
			return &it, nil
		}
	}
	return nil, fmt.Errorf("couldn't find user %d", id)
}
