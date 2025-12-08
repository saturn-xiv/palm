package app

import (
	"embed"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"

	"github.com/BurntSushi/toml"
	"github.com/goccy/go-yaml"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/daisy/iso4217"
	"github.com/saturn-xiv/palm/daisy/models"
)

type I18nSyncConfig struct {
	Database *Database `toml:"database"`
}

func DbSeeds(config_file string, folders []string, debug bool) error {
	slog.Debug("load configuration from", "file", config_file)
	var config I18nSyncConfig
	if _, err := toml.DecodeFile(config_file, &config); err != nil {
		return err
	}
	if _, err := iso4217.Iso4217(); err != nil {
		return err
	}
	db, err := config.Database.Open(debug)
	if err != nil {
		return err
	}
	if err = db.Transaction(func(tx *gorm.DB) error {
		if err := load_locale_from_embed(tx); err != nil {
			return err
		}
		for _, it := range folders {
			if err := load_locale_from_folder(tx, it); err != nil {
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

func load_locale_from_folder(db *gorm.DB, folder string) error {
	slog.Info("load locales from", "folder", folder)
	entries, err := os.ReadDir(folder)

	if err != nil {
		return err
	}
	var languages []string
	for _, entry := range entries {
		if entry.Type().IsDir() {
			slog.Debug("found", "language", entry.Name())
			languages = append(languages, entry.Name())
		}
	}
	for _, entry := range entries {
		if entry.Type().IsRegular() {
			for _, lang := range languages {
				load_locale_from_file(db, lang, filepath.Join(folder, entry.Name()))
			}
		}
	}

	for _, lang := range languages {
		entries, err := os.ReadDir(filepath.Join(folder, lang))
		if err != nil {
			return err
		}
		for _, entry := range entries {
			if entry.Type().IsRegular() {
				load_locale_from_file(db, lang, filepath.Join(folder, lang, entry.Name()))
			}
		}
	}
	return nil
}

func load_locale_from_file(db *gorm.DB, lang string, file string) error {
	slog.Debug("load file from", "lang", lang, "file", file)
	buf, err := os.ReadFile(file)
	if err != nil {
		return err
	}
	return load_locale_with_file(db, lang, file, buf)
}

func load_locale_with_file(db *gorm.DB, lang string, file string, content []byte) error {
	ext := filepath.Ext(file)
	zone := filepath.Base(strings.TrimSuffix(file, ext))
	switch ext {
	case ".yml":
		return load_locale_from_yaml(db, lang, zone, content)
	case ".yaml":
		return load_locale_from_yaml(db, lang, zone, content)
	default:
		return fmt.Errorf("unsupported file type %s", ext)
	}
}

func load_locale_from_yaml(db *gorm.DB, lang string, zone string, content []byte) error {
	slog.Debug("load yaml records", "lang", lang, "zone", zone)

	tree := make(map[string]interface{})
	if err := yaml.Unmarshal(content, &tree); err != nil {
		return err
	}
	return load_locale_from_map(db, lang, zone, tree)
}

func load_locale_from_map(db *gorm.DB, lang string, zone string, tree map[string]interface{}) error {
	for key, val := range tree {
		k := fmt.Sprintf("%s.%s", zone, key)
		if s, ok := val.(string); ok {
			if err := create_locale(db, lang, k, s); err != nil {
				return err
			}
			continue
		}
		if m, ok := val.(map[string]interface{}); ok {
			if err := load_locale_from_map(db, lang, k, m); err != nil {
				return err
			}
			continue
		}
		return fmt.Errorf("unsupported record %s.%s", lang, zone)
	}
	return nil
}

func create_locale(db *gorm.DB, lang string, code string, message string) error {
	{
		_, err := models.LocaleByLangAndCode(db, lang, code)
		if err == nil {
			return nil
		}
		if !errors.Is(err, gorm.ErrRecordNotFound) {
			return err
		}
	}

	if err := db.Create(&models.Locale{
		Lang:    lang,
		Code:    code,
		Message: message,
	}).Error; err != nil {
		return err
	}
	return nil
}

//go:embed locales/* locales/*/*
var gl_locales_files embed.FS

func load_locale_from_embed(db *gorm.DB) error {
	root := "locales"
	entries, err := gl_locales_files.ReadDir(root)
	if err != nil {
		return err
	}
	var languages []string
	for _, entry := range entries {
		if entry.Type().IsDir() {
			slog.Debug("found", "language", entry.Name())
			languages = append(languages, entry.Name())
		}
	}
	for _, entry := range entries {
		if entry.Type().IsRegular() {
			for _, lang := range languages {
				load_locale_from_embed_file(db, lang, filepath.Join(root, entry.Name()))
			}
		}
	}

	for _, lang := range languages {
		entries, err := gl_locales_files.ReadDir(filepath.Join(root, lang))
		if err != nil {
			return err
		}
		for _, entry := range entries {
			if entry.Type().IsRegular() {
				load_locale_from_embed_file(db, lang, filepath.Join(root, lang, entry.Name()))
			}
		}
	}
	return nil
}

func load_locale_from_embed_file(db *gorm.DB, lang string, file string) error {
	slog.Debug("load embed", "lang", lang, "file", file)
	buf, err := gl_locales_files.ReadFile(file)
	if err != nil {
		return err
	}
	return load_locale_with_file(db, lang, file, buf)
}
