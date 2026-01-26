package graphql

import (
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

func RoleByCode(code string) (string, error) {
	switch code {
	case v2.ROLE_ADMINISTRATOR:
		return v2.RoleAdministrator().ToString()
	case v2.ROLE_ROOT:
		return v2.RoleRoot().ToString()
	default:
		return v2.RoleByCode(models.ToCode(code)).ToString()
	}
}
