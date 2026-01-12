package app

import (
	"fmt"
	"log/slog"
	"os/user"

	"github.com/BurntSushi/toml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/env"
	"github.com/saturn-xiv/palm/daisy/models"
	v2 "github.com/saturn-xiv/palm/daisy/portal/v2"
)

type ResetPasswordForEmailUserConfig struct {
	Database *Database `toml:"database"`
}

func ResetPasswordForEmailUser(config_file string, email string, password string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config ResetPasswordForEmailUserConfig
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
		form := models.NewResetPasswordForEmailUserForm(email, password)
		if err := form.Execute(tx, hmac); err != nil {
			return err
		}
		var user models.EmailUser
		if err := tx.Where("email = ?", form.Email).First(&user).Error; err != nil {
			return err
		}
		if err := models.CreateLog(tx, user.UserID, plugin, gl_localhost, v2.Log_DEBUG, fmt.Sprintf("reset password by %s", cur_usr.Username)); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}

	slog.Info("done.")
	return nil
}
