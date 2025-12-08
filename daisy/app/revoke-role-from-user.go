package app

import (
	"fmt"
	"log/slog"

	"gorm.io/gorm"

	auth_v2 "github.com/saturn-xiv/palm/daisy/auth/v2"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/graphql"
	"github.com/saturn-xiv/palm/daisy/models"
)

func RevokeRoleFromUser(config_file string, user_sn string, role_code string, debug bool) error {
	db, enforcer, admin, err := open_user_role_config(config_file, debug)
	if err != nil {
		return err
	}
	role_s, err := graphql.RoleByCode(role_code)
	if err != nil {
		return err
	}
	if err = db.Transaction(func(tx *gorm.DB) error {
		user, user_s, err := graphql.UserBySn(db, user_sn)
		if err != nil {
			return err
		}
		if _, err = enforcer.DeleteRoleForUser(user_s, role_s); err != nil {
			return err
		}
		if err := models.CreateLog(tx, user.ID, env.Plugin(), gl_localhost, auth_v2.Log_Debug, fmt.Sprintf("revoked role %s by %s", role_code, admin.Username)); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}
