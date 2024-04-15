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
	db    *gorm.DB
	items map[string]map[string]string
}

func (p *I18n) T(lang string, code string) (string, error) {
	if tmp, ok := p.items[lang]; ok {
		if val, ok := tmp[code]; ok {
			return val, nil
		}
	}
	it, err := models.GetLocaleByLangAndCode(p.db, lang, code)
	if err != nil {
		return "", err
	}
	return it.Message, nil
}

func (p *I18n) Tr(lang string, code string, data any) string {
	name := fmt.Sprintf("%s.%s", lang, code)
	text, err := p.T(lang, code)
	if err != nil {
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
	items := make(map[string]map[string]string)
	root := "locales"
	dirs, err := gl_locales_fs.ReadDir(root)
	if err != nil {
		return nil, err
	}
	for _, il := range dirs {
		if !il.IsDir() {
			continue
		}
		lang := il.Name()
		l_m := make(map[string]string)

		files, err := gl_locales_fs.ReadDir(filepath.Join(root, lang))
		if err != nil {
			return nil, err
		}
		for _, it := range files {
			slog.Debug("find", slog.String("language", lang), slog.String("file", it.Name()))
			buf, err := gl_locales_fs.ReadFile(filepath.Join(root, lang, it.Name()))
			if err != nil {
				return nil, err
			}
			tmp, err := load_ini(buf)
			if err != nil {
				return nil, err
			}
			maps.Copy(l_m, tmp)
		}
		items[lang] = l_m
	}
	for _, it := range dirs {
		if !it.IsDir() {
			slog.Debug("load global locale", slog.String("file", it.Name()))
			buf, err := gl_locales_fs.ReadFile(filepath.Join(root, it.Name()))
			if err != nil {
				return nil, err
			}
			tmp, err := load_ini(buf)
			if err != nil {
				return nil, err
			}
			for _, val := range items {
				maps.Copy(val, tmp)
			}

		}
	}
	// for l, m := range items {
	// 	for k, v := range m {
	// 		slog.Debug("", slog.String("lang", l), slog.String(k, v))
	// 	}
	// }
	return &I18n{
		db:    db,
		items: items,
	}, nil
}

func load_ini(buf []byte) (map[string]string, error) {
	items := make(map[string]string)
	file, err := ini.Load(buf)
	if err != nil {
		return nil, err
	}
	for _, sec := range file.Sections() {
		for k, v := range sec.KeysHash() {
			key := fmt.Sprintf("%s.%s", sec.Name(), k)
			items[key] = v
		}
	}
	return items, nil
}
