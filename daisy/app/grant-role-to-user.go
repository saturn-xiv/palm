package app

import (
	"fmt"
	"log/slog"
	"os/user"

	"github.com/BurntSushi/toml"
	"github.com/casbin/casbin/v3"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/cache"
	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/graphql"
	"github.com/saturn-xiv/palm/daisy/models"
	auth_v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
	"github.com/saturn-xiv/palm/daisy/rbac"
	rbac_v2 "github.com/saturn-xiv/palm/daisy/rbac/v2"
)

type UserRoleConfig struct {
	Database *Database           `toml:"database"`
	Redis    *cache.RedisCluster `toml:"redis"`
}

func open_user_role_config(config_file string, debug bool) (*gorm.DB, *casbin.Enforcer, *user.User, error) {
	slog.Debug("load configuration from", "file", config_file)
	var config UserRoleConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return nil, nil, nil, err
	}
	db, err := config.Database.Open(debug)
	if err != nil {
		return nil, nil, nil, err
	}
	enforcer, err := rbac.NewEnforcer(db, config.Redis.Addresses(), config.Redis.Namespace)
	if err != nil {
		return nil, nil, nil, err
	}

	usr, err := user.Current()
	if err != nil {
		return nil, nil, nil, err
	}
	return db, enforcer, usr, nil
}

func GrantRoleToUser(config_file string, user_sn string, role_code string, debug bool) error {
	db, enforcer, admin, err := open_user_role_config(config_file, debug)
	if err != nil {
		return err
	}
	role_s, err := graphql.RoleByCode(role_code)
	if err != nil {
		return err
	}
	if err = db.Transaction(func(tx *gorm.DB) error {
		user, err := models.UserBySn(db, user_sn)
		if err != nil {
			return err
		}
		user_ := rbac_v2.Subject{
			By: &rbac_v2.Subject_User_{
				User: &rbac_v2.Subject_User{
					By: &rbac_v2.Subject_User_Id{
						Id: int64(user.ID),
					},
				},
			},
		}
		user_s, err := user_.ToString()
		if err != nil {
			return err
		}
		if _, err = enforcer.AddRoleForUser(user_s, role_s); err != nil {
			return err
		}
		if err := models.CreateLog(tx, user.ID, env.Plugin(), gl_localhost, auth_v2.UserIndexLogResponse_Item_WARNING, fmt.Sprintf("granted role %s by %s", role_code, admin.Username)); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}
