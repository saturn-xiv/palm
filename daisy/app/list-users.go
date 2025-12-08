package app

import (
	"fmt"
	"log/slog"

	"github.com/BurntSushi/toml"

	"github.com/saturn-xiv/palm/daisy/models"
)

type ListUsersConfig struct {
	Database *Database `toml:"database"`
}

func ListUsers(config_file string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config ListUsersConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	db, err := config.Database.Open(debug)
	if err != nil {
		return err
	}
	{
		var items []models.EmailUser
		if err := db.Order("email ASC").Preload("User").Find(&items).Error; err != nil {
			return err
		}
		fmt.Printf("%-36s %-32s %-32s\n", "SN", "EMAIL", "NAME")
		for _, it := range items {
			fmt.Printf("%s %-32s %-32s\n", it.User.Sn, it.Email, it.Name)
		}
	}
	{
		var items []models.GoogleOauth2User
		if err := db.Order("email ASC").Preload("User").Find(&items).Error; err != nil {
			return err
		}
		fmt.Printf("%-36s %-32s %-32s\n", "SN", "EMAIL", "NAME")
		for _, it := range items {
			fmt.Printf("%s %-32s %-32s\n", it.User.Sn, it.Email, it.Name)
		}
	}
	return nil
}
