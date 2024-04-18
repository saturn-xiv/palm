package models

import (
	"time"

	"github.com/casbin/casbin/v2"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

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
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Root_{},
	})
}

func (p *User) IsAdministrator(enforcer *casbin.Enforcer) error {
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Administrator_{},
	})
}
func (p *User) Has(enforcer *casbin.Enforcer, role string) error {
	return p.has(enforcer, &pb.PolicyRolesResponse_Item{
		By: &pb.PolicyRolesResponse_Item_Member{
			Member: role,
		},
	})
}

func (p *User) has(enforcer *casbin.Enforcer, role *pb.PolicyRolesResponse_Item) error {
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
	return p.can(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
		},
	})
}

func (p *User) Can_(enforcer *casbin.Enforcer, operation string, resource_type string, resource_id uint32) error {
	return p.can(enforcer, &pb.PolicyPermissionsResponse_Item{
		Operation: operation,
		Resource: &pb.PolicyPermissionsResponse_Item_Resource{
			Type: resource_type,
			Id:   &resource_id,
		},
	})
}

func (p *User) can(enforcer *casbin.Enforcer, permission *pb.PolicyPermissionsResponse_Item) error {
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
