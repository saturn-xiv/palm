package models

import (
	"fmt"
	"log/slog"
	"os"
	"path"
	"reflect"
	"strings"
	"time"

	"gorm.io/gorm"

	"github.com/BurntSushi/toml"
	"github.com/saturn-xiv/palm/jasmine/web"
)

type Locale struct {
	ID        uint32 `gorm:"primaryKey"`
	Lang      string
	Code      string
	Message   string
	UpdatedAt time.Time
	CreatedAt time.Time
}

func (Locale) TableName() string {
	return "locales"
}

type localeItem struct {
	lang    string
	code    string
	message string
}

func Languages(db *gorm.DB) ([]string, error) {
	items := []string{}
	if err := db.Model(&Locale{}).Distinct("lang").Order("lang ASC").Find(items).Error; err != nil {
		return nil, err
	}
	return items, nil
}

func LoadLocalesByToml(db *gorm.DB, folder string) (int, int, error) {
	items, err := load_locales_folder_by_toml(folder)
	if err != nil {
		return 0, 0, err
	}
	inserted := 0
	now := time.Now()
	for _, it := range items {
		var c int64
		if err = db.Model(&Locale{}).Where("lang = ? AND code = ?", it.lang, it.code).Count(&c).Error; err != nil {
			return len(items), 0, err
		}
		if c > 0 {
			slog.Debug("ignore", slog.String("lang", it.lang), slog.String("code", it.code))
			continue
		}
		slog.Debug("create", slog.String("lang", it.lang), slog.String("code", it.code))
		db.Create(&Locale{Lang: it.lang, Code: it.code, Message: it.message, UpdatedAt: now})
		inserted = inserted + 1
	}
	return len(items), inserted, nil
}

func load_locales_from_file_by_toml(file string, lang string, namespace string) ([]localeItem, error) {
	slog.Debug("load", slog.String("file", file), slog.String("lang", lang), slog.String("namespace", namespace))
	var object web.H
	_, err := toml.DecodeFile(file, &object)
	if err != nil {
		return nil, err
	}
	items := load_locales_from_hash(object, lang, namespace)
	return items, nil
}

func load_locales_from_hash(object web.H, lang string, namespace string) []localeItem {
	var items []localeItem
	for k, v := range object {
		code := fmt.Sprintf("%s.%s", namespace, k)
		if s, ok := v.(string); ok {
			slog.Debug("found", slog.String("lang", lang), slog.String("code", code))
			items = append(items, localeItem{lang: lang, code: code, message: s})
			continue
		}
		if o, ok := v.(map[string]interface{}); ok {
			tmp := load_locales_from_hash(o, lang, code)
			items = append(items, tmp...)
			continue
		}
		slog.Warn("ignore record", slog.String("lang", lang), slog.String("key", k), slog.String("type", reflect.TypeOf(v).String()))
	}
	return items
}

func load_locales_folder_by_toml(folder string) ([]localeItem, error) {
	slog.Debug("load locales from", slog.String("folder", folder))

	entries, err := os.ReadDir(folder)
	if err != nil {
		return nil, err
	}
	languages := make([]string, 0)
	for _, entry := range entries {
		it := entry.Name()
		if entry.IsDir() {
			slog.Debug("found", slog.String("language", it))
			languages = append(languages, it)
		}
	}

	var items []localeItem
	for _, entry := range entries {
		if !entry.IsDir() {
			for _, lang := range languages {
				file := entry.Name()
				if ns, _, err := detect_namespace_by_file(file); err == nil {
					tmp, err := load_locales_from_file_by_toml(path.Join(folder, file), lang, ns)
					if err != nil {
						return nil, err
					}
					items = append(items, tmp...)
				}
			}
		}
	}
	for _, lang := range languages {
		folder := path.Join(folder, lang)
		entries, err := os.ReadDir(folder)
		if err != nil {
			return nil, err
		}
		for _, entry := range entries {
			if !entry.IsDir() {
				file := entry.Name()
				if ns, _, err := detect_namespace_by_file(file); err == nil {
					tmp, err := load_locales_from_file_by_toml(path.Join(folder, file), lang, ns)
					if err != nil {
						return nil, err
					}
					items = append(items, tmp...)
				}
			}
		}
	}

	return items, nil
}

func detect_namespace_by_file(file string) (string, string, error) {
	ext := path.Ext(file)
	if ext == ".toml" {
		return strings.TrimSuffix(file, ext), ext, nil
	}
	return "", ext, fmt.Errorf("unsupported filetype %s", ext)
}
