package db

import (
	"context"
	"log/slog"

	"github.com/BurntSushi/toml"
	"github.com/redis/go-redis/v9"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/jasmine/services/portal/models"
)

func Seed(config_file string, locales_folder string) error {
	slog.Debug("load configuration from", slog.String("file", config_file))
	var config Config
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}

	db, err := config.Database.Open()
	if err != nil {
		return err
	}
	if err = db.Transaction(func(tx *gorm.DB) error {
		if len(locales_folder) > 0 {
			if err := load_locales(tx, locales_folder); err != nil {
				return err
			}
		}
		if err := load_currencies(tx); err != nil {
			return err
		}
		return nil
	}); err != nil {
		return err
	}

	ctx := context.Background()
	cache, err := config.Redis.Open(ctx)
	if err != nil {
		return err
	}

	if err = set_sample_page(cache); err != nil {
		return err
	}

	slog.Info("done.")
	return nil
}

func set_sample_page(*redis.ClusterClient) error {
	slog.Info("setup sample page /sample.html")
	// TODO
	return nil
}

func load_locales(db *gorm.DB, folder string) error {
	total, inserted, err := models.LoadLocalesByToml(db, folder)
	if err != nil {
		return err
	}
	slog.Info("locales", slog.Int("found", total), slog.Int("inserted", inserted))
	return nil
}

func load_currencies(db *gorm.DB) error {
	var count int64
	if err := db.Model(&models.Currency{}).Count(&count).Error; err != nil {
		return err
	}
	if count > 0 {
		slog.Warn("currency tables isn't empty, skipped")
		return nil
	}
	total, inserted, err := models.LoadIso4217ListOne(db)
	if err != nil {
		return err
	}
	slog.Info("currencies", slog.Int("found", total), slog.Int("inserted", inserted))
	return nil
}
