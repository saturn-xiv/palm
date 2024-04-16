package i18n

import (
	"bytes"
	"embed"
	"fmt"
	"log/slog"
	"maps"
	"path/filepath"
	"text/template"

	"gopkg.in/ini.v1"
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/models"
)

//go:embed locales/*
var gl_locales_fs embed.FS

type I18n struct {
	items map[string]string
}

func (p *I18n) Tr(lang string, code string, data any) string {
	name := fmt.Sprintf("%s.%s", lang, code)
	text, ok := p.items[name]
	if !ok {
		return name
	}
	tpl, err := template.New(name).Parse(text)
	if err != nil {
		slog.Error(err.Error())
		return name
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, data); err != nil {
		slog.Error(err.Error())
		return name
	}
	return buf.String()
}

func New(db *gorm.DB) (*I18n, error) {
	items := make(map[string]string)
	root := "locales"
	dirs, err := gl_locales_fs.ReadDir(root)
	if err != nil {
		return nil, err
	}
	var languages []string
	for _, il := range dirs {
		if !il.IsDir() {
			continue
		}
		lang := il.Name()
		languages = append(languages, lang)

		files, err := gl_locales_fs.ReadDir(filepath.Join(root, lang))
		if err != nil {
			return nil, err
		}
		for _, it := range files {
			slog.Debug("load locale-ini", slog.String("language", lang), slog.String("file", it.Name()))
			buf, err := gl_locales_fs.ReadFile(filepath.Join(root, lang, it.Name()))
			if err != nil {
				return nil, err
			}
			tmp, err := load_ini(lang, buf)
			if err != nil {
				return nil, err
			}
			maps.Copy(items, tmp)
		}
	}
	for _, it := range dirs {
		if !it.IsDir() {
			for _, lang := range languages {
				slog.Debug("load locale-ini", slog.String("language", lang), slog.String("file", it.Name()))
				buf, err := gl_locales_fs.ReadFile(filepath.Join(root, it.Name()))
				if err != nil {
					return nil, err
				}
				tmp, err := load_ini(lang, buf)
				if err != nil {
					return nil, err
				}
				maps.Copy(items, tmp)
			}

		}
	}

	{
		var tmp []models.Locale
		if rst := db.Select("lang", "code", "message").Find(&tmp); rst.Error != nil {
			return nil, rst.Error
		}
		for _, it := range tmp {
			items[it.Key()] = it.Message
		}
	}
	return &I18n{
		items: items,
	}, nil
}

func load_ini(lang string, buf []byte) (map[string]string, error) {
	items := make(map[string]string)
	file, err := ini.Load(buf)
	if err != nil {
		return nil, err
	}
	for _, sec := range file.Sections() {
		for k, v := range sec.KeysHash() {
			key := fmt.Sprintf("%s.%s.%s", lang, sec.Name(), k)
			items[key] = v
		}
	}
	return items, nil
}
