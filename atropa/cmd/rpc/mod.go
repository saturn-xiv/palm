package rpc

import (
	"embed"
	"encoding/json"
	"fmt"
	"log/slog"
	"path/filepath"
	"reflect"

	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/atropa/balsam/models"
	"github.com/saturn-xiv/palm/atropa/env"
	"github.com/saturn-xiv/palm/atropa/env/minio"
	"github.com/saturn-xiv/palm/atropa/env/rabbitmq"
	"github.com/saturn-xiv/palm/atropa/env/redis"
	wechat_mini_program "github.com/saturn-xiv/palm/atropa/env/wechat-mini-program"
	wechat_oauth2 "github.com/saturn-xiv/palm/atropa/env/wechat-oauth2"
	wechat_pay "github.com/saturn-xiv/palm/atropa/env/wechat-pay"
)

type Config struct {
	KeysDir           string                      `toml:"keys-dir"`
	GoogleOauth2      *GoogleOauth2               `toml:"google-oauth2,omitempty"`
	WechatOauth2      *wechat_oauth2.Config       `toml:"wechat-oauth2,omitempty"`
	WechatMiniProgram *wechat_mini_program.Config `toml:"wechat-mini-program,omitempty"`
	WechatPay         *wechat_pay.Config          `toml:"wechat-mini-pay,omitempty"`
	Redis             redis.Cluster               `toml:"redis"`
	Database          env.Database                `toml:"database"`
	RabbitMQ          rabbitmq.Config             `toml:"rabbitmq"`
	Minio             minio.Config                `toml:"minio"`
	Tls               env.Tls                     `toml:"tls"`
}

type GoogleOauth2 struct {
	ProjectID   string `toml:"project-id"`
	RedirectURL string `toml:"redirect-url"`
}

//go:embed locales/*
var gl_locales_fs embed.FS

func i18n_sync(db *gorm.DB) error {
	slog.Info("load locales into db")
	root := "locales"
	entries, err := gl_locales_fs.ReadDir(root)
	if err != nil {
		return err
	}
	for _, entry := range entries {
		if entry.IsDir() {
			lang := entry.Name()
			slog.Debug("found", slog.String("lang", lang))
			files, err := gl_locales_fs.ReadDir(filepath.Join(root, lang))
			if err != nil {
				return err
			}
			for _, file := range files {
				if err = load_locales_from_json_file(db, lang, filepath.Join(root, lang, file.Name()), file.Name()); err != nil {
					return err
				}
			}
		}
	}
	languages, err := models.Languages(db)
	if err != nil {
		return err
	}
	for _, lang := range languages {
		for _, entry := range entries {
			if entry.Type().IsRegular() {
				slog.Debug("found global locale file", slog.String("name", entry.Name()))
				if err = load_locales_from_json_file(db, lang, filepath.Join(root, entry.Name()), entry.Name()); err != nil {
					return err
				}
			}
		}
	}
	return nil
}

func load_locales_from_json_file(db *gorm.DB, lang string, file string, namespace string) error {
	buf, err := gl_locales_fs.ReadFile(file)
	if err != nil {
		return err
	}
	tmp := make(map[string]interface{})
	if err := json.Unmarshal(buf, &tmp); err != nil {
		return err
	}
	return load_locales_from_json_map(db, lang, namespace, tmp)
}

func load_locales_from_json_map(db *gorm.DB, lang string, namespace string, node map[string]interface{}) error {
	for c, v := range node {
		c = fmt.Sprintf("%s.%s", namespace, c)
		if s, ok := v.(string); ok {
			if err := db.Where("lang = ? AND code = ?", lang, c).First(&models.Locale{}).Error; err == gorm.ErrRecordNotFound {
				if err = db.Create(&models.Locale{
					Lang:    lang,
					Code:    c,
					Message: s,
				}).Error; err != nil {
					return err
				}
			}
		} else if m, ok := v.(map[string]interface{}); ok {
			if err := load_locales_from_json_map(db, lang, c, m); err != nil {
				return err
			}
		} else {
			return fmt.Errorf("bad locales type %s", reflect.TypeOf(v).String())
		}
	}
	return nil
}
