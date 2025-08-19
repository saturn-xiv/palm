package user

import (
	"log/slog"
	"strings"

	"github.com/BurntSushi/toml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/env"
	"github.com/saturn-xiv/palm/jasmine/env/crypto"
	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
	v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"github.com/saturn-xiv/palm/jasmine/web"
)

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

func AddRoleForEmailUser(config_file string, email string, role string) error {
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err := db.Transaction(func(tx *gorm.DB) error {
		return nil
	}); err != nil {
		return err
	}
	return nil
}

func DeleteRoleForEmailUser(config_file string, email string, role string) error {
	slog.Debug("load configuration", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err := db.Transaction(func(tx *gorm.DB) error {
		return nil
	}); err != nil {
		return err
	}
	return nil
}
