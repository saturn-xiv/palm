package app

import (
	"fmt"
	"log/slog"
	"os/user"
	"time"

	"github.com/BurntSushi/toml"
	"golang.org/x/text/language"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type CreateUserByEmailConfig struct {
	Database *Database `toml:"database"`
}

func CreateUserByEmail(config_file string, name string, email string, password string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config CreateUserByEmailConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	_, hmac, _, err := open_secrets()
	if err != nil {
		return err
	}
	db, err := config.Database.Open(debug)
	if err != nil {
		return err
	}

	plugin := env.Plugin()
	cur_usr, err := user.Current()
	if err != nil {
		return err
	}

	if err = db.Transaction(func(tx *gorm.DB) error {
		form := models.NewCreateEmailByUserForm(name, email, password)
		if err := form.Execute(tx, hmac, &language.AmericanEnglish, time.UTC); err != nil {
			return err
		}
		var user models.EmailUser
		if err := tx.Where("email = ?", form.Email).Preload("User").First(&user).Error; err != nil {
			return err
		}
		if err := models.ConfirmEmailUser(tx, user.ID); err != nil {
			return err
		}
		if err := models.CreateLog(tx, user.UserID, plugin, gl_localhost, v2.Log_DEBUG, fmt.Sprintf("created by %s", cur_usr.Username)); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}

	slog.Info("done.")
	return nil
}
