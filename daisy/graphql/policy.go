package graphql

import (
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
	"gorm.io/gorm"
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

func UserBySn(db *gorm.DB, sn string) (*models.User, string, error) {
	var user models.User
	if err := db.Where("sn = ?", sn).First(&user).Error; err != nil {
		return nil, "", err
	}
	subject, err := v2.UserById(uint32(user.ID)).ToString()
	if err != nil {
		return nil, "", err
	}
	return &user, subject, nil
}
