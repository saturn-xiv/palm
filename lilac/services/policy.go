package services

import (
	"github.com/casbin/casbin/v2"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/env/crypto"
	pb "github.com/saturn-xiv/palm/lilac/services/v2"
)

type PolicyService struct {
	pb.UnimplementedPolicyServer

	db       *gorm.DB
	jwt      *crypto.Jwt
	enforcer *casbin.Enforcer
}

// rpc Users(google.protobuf.Empty) returns (PolicyUsersResponse) {}
//   rpc Roles(google.protobuf.Empty) returns (PolicyRolesResponse) {}

//   rpc AddRolesForUser(PolicyRolesForUserRequest)
//       returns (google.protobuf.Empty) {}
//   rpc DeleteRolesForUser(PolicyRolesForUserRequest)
//       returns (google.protobuf.Empty) {}
//   rpc GetRolesForUser(PolicyUserRequest) returns (PolicyRolesResponse) {}

//   rpc AddUsersForRole(PolicyUsersForRoleRequest)
//       returns (google.protobuf.Empty) {}
//   rpc DeleteUsersForRole(PolicyUsersForRoleRequest)
//       returns (google.protobuf.Empty) {}
//   rpc GetUsersForRole(PolicyRolesResponse.Item) returns (PolicyUsersResponse) {}
//   rpc DeleteRole(PolicyRoleRequest) returns (google.protobuf.Empty) {}

//   rpc AddPermissionsForUser(PolicyPermissionsForUserRequest)
//       returns (google.protobuf.Empty) {}
//   rpc DeletePermissionsForUser(PolicyPermissionsForUserRequest)
//       returns (google.protobuf.Empty) {}
//   rpc GetPermissionsForUser(PolicyUserRequest) returns (google.protobuf.Empty) {
//   }

//   rpc AddPermissionsForRole(PolicyPermissionsForRoleRequest)
//       returns (google.protobuf.Empty) {}
//   rpc DeletePermissionsForRole(PolicyPermissionsForRoleRequest)
//       returns (google.protobuf.Empty) {}
//   rpc GetPermissionsForRole(PolicyRoleRequest) returns (google.protobuf.Empty) {
//   }

func NewPolicyService(db *gorm.DB, jwt *crypto.Jwt, enforcer *casbin.Enforcer) *PolicyService {
	return &PolicyService{db: db, jwt: jwt, enforcer: enforcer}
}
