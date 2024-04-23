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
	items map[string]map[string]string
}

func (p *I18n) Tr(lang string, code string, data any) string {
	key := fmt.Sprintf("%s.%s", lang, code)
	tmp, ok := p.items[lang]
	if !ok {
		return key
	}
	text, ok := tmp[code]
	if !ok {
		return key
	}
	tpl, err := template.New(key).Parse(text)
	if err != nil {
		slog.Error(err.Error())
		return key
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, data); err != nil {
		slog.Error(err.Error())
		return key
	}
	return buf.String()
}

func (p *I18n) ByLang(lang string) (map[string]string, bool) {
	tmp, ok := p.items[lang]
	return tmp, ok
}

func (p *I18n) Reload(db *gorm.DB) error {
	return reload(db, p.items)
}

func New(db *gorm.DB) (*I18n, error) {
	items := make(map[string]map[string]string)
	if err := reload(db, items); err != nil {
		return nil, err
	}

	return &I18n{
		items: items,
	}, nil
}
func reload(db *gorm.DB, items map[string]map[string]string) error {

	root := "locales"
	dirs, err := gl_locales_fs.ReadDir(root)
	if err != nil {
		return err
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
			return err
		}
		by_lang := make(map[string]string)
		for _, it := range files {
			slog.Debug("load locale-ini", slog.String("language", lang), slog.String("file", it.Name()))
			buf, err := gl_locales_fs.ReadFile(filepath.Join(root, lang, it.Name()))
			if err != nil {
				return err
			}
			tmp, err := load_from_ini(buf)
			if err != nil {
				return err
			}
			maps.Copy(by_lang, tmp)
		}
		items[lang] = by_lang
	}
	for _, it := range dirs {
		if !it.IsDir() {
			for _, lang := range languages {
				slog.Debug("load locale-ini", slog.String("language", lang), slog.String("file", it.Name()))
				buf, err := gl_locales_fs.ReadFile(filepath.Join(root, it.Name()))
				if err != nil {
					return err
				}
				tmp, err := load_from_ini(buf)
				if err != nil {
					return err
				}
				by_lang := items[lang]
				maps.Copy(by_lang, tmp)
			}
		}
	}

	if err = load_from_db(db, items); err != nil {
		return err
	}

	for lng, it := range items {
		slog.Debug("found", slog.String("language", lng), slog.Int("count", len(it)))
	}
	return nil
}
func load_from_db(db *gorm.DB, items map[string]map[string]string) error {

	var tmp []models.Locale
	if rst := db.Select("lang", "code", "message").Find(&tmp); rst.Error != nil {
		return rst.Error
	}
	for _, it := range tmp {
		items[it.Lang][it.Code] = it.Message
	}
	return nil
}

func load_from_ini(buf []byte) (map[string]string, error) {
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
