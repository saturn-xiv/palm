package user

import (
	"fmt"
	"log/slog"
	"strings"

	"github.com/BurntSushi/toml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	casbin_v2 "github.com/saturn-xiv/palm/jasmine/services/casbin/v2"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

func SetEmailUserPassword(config_file string, email string, password string) error {
	slog.Info("set user password", slog.String("email", email))
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, mac, _, err := crypto.Open(config.SecretsStore)
	if err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err := db.Transaction(func(tx *gorm.DB) error {
		e_user, err := models.UserByEmail(db, web.ToCode(email))
		if err != nil {
			return err
		}
		form := models.SetEmailUserPasswordForm{Password: password}
		if err := form.Execute(tx, mac, e_user.ID); err != nil {
			return err
		}
		{
			it := models.CreateLogForm{
				Plugin:       env.PLUGIN_NAME,
				IP:           models.LOCALHOST,
				Message:      "created by system administrator",
				ResourceType: web.ResourceType((*models.EmailUser)(nil)),
			}
			if err = it.Execute(tx, e_user.UserID, v2.UserIndexLogResponse_Item_INFO, &e_user.ID); err != nil {
				return err
			}
		}
		return nil
	}); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}

func CreateEmailUser(config_file string, email string, name string, password string) error {
	slog.Info("create user", slog.String("name", name), slog.String("email", email))
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, mac, _, err := crypto.Open(config.SecretsStore)
	if err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err := db.Transaction(func(tx *gorm.DB) error {
		form := models.CreateEmailUserForm{
			Email:    web.ToCode(email),
			RealName: strings.TrimSpace(name),
			Password: password,
		}
		{
			exists, err := models.IsEmailUserExists(db, form.Email)
			if err != nil {
				return err
			}
			if exists {
				slog.Warn("user already exists", slog.String("email", form.Email))
				return nil
			}
		}
		if err := form.Execute(tx, mac, models.EN_US, models.UTC); err != nil {
			return err
		}
		e_user, err := models.UserByEmail(tx, form.Email)
		if err != nil {
			return err
		}
		if err = models.ConfirmEmailUser(tx, e_user.ID); err != nil {
			return err
		}
		{
			it := models.CreateLogForm{
				Plugin:       env.PLUGIN_NAME,
				IP:           models.LOCALHOST,
				Message:      "created by system administrator",
				ResourceType: web.ResourceType((*models.EmailUser)(nil)),
			}
			if err = it.Execute(tx, e_user.UserID, v2.UserIndexLogResponse_Item_INFO, &e_user.ID); err != nil {
				return err
			}
		}
		return nil
	}); err != nil {
		return err
	}
	slog.Info("done.")
	return nil
}

func RoleForEmailUser(config_file string, email string, role string, apply bool) error {
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	e_user, err := models.UserByEmail(db, web.ToCode(email))
	if err != nil {
		return err
	}
	u_subject, err := e_user.Subject()
	if err != nil {
		return err
	}

	r_subject, err := web.ProtoBufMessageToString(to_role_subject(role))
	if err != nil {
		return err
	}
	enf, err := env.OpenCasbinEnforcer(env.PLUGIN_NAME, db, config.Redis.Addrs())
	if err != nil {
		return nil
	}

	ok := false
	if apply {
		if ok, err = enf.AddRoleForUser(u_subject, r_subject); err != nil {
			return err
		}
	} else {
		if ok, err = enf.DeleteRoleForUser(u_subject, r_subject); err != nil {
			return err
		}
	}

	if ok {
		if err := db.Transaction(func(tx *gorm.DB) error {
			it := models.CreateLogForm{
				Plugin:       env.PLUGIN_NAME,
				IP:           models.LOCALHOST,
				ResourceType: web.ResourceType((*models.EmailUser)(nil)),
			}
			if apply {
				it.Message = fmt.Sprintf("apply role(%s) by system administrator", role)
			} else {
				it.Message = fmt.Sprintf("relief role(%s) by system administrator", role)
			}
			if err = it.Execute(tx, e_user.UserID, v2.UserIndexLogResponse_Item_INFO, &e_user.ID); err != nil {
				return err
			}
			return nil
		}); err != nil {
			return err
		}
	}
	slog.Info("done.")
	return nil
}

func to_role_subject(code string) *casbin_v2.Subject {
	switch code {
	case "root":
		return casbin_v2.NewRootRoleSubject()
	case "administrator":
		return casbin_v2.NewAdministratorRoleSubject()
	default:
		return casbin_v2.NewRoleSubjectByCode(web.ToCode(code))
	}
}
