package models

import (
	"bytes"
	"fmt"
	"text/template"

	"gorm.io/gorm"
)

func Languages(db *gorm.DB) ([]string, error) {
	var items []string
	if err := db.Model(&Locale{}).Distinct().Pluck("lang", &items).Error; err != nil {
		return nil, err
	}
	return items, nil
}

func SetLocale(db *gorm.DB, lang string, code string, message string) error {
	var it Locale
	err := db.Where("lang = ? AND code = ?", lang, code).First(&it).Error
	if err == gorm.ErrRecordNotFound {
		return db.Create(&Locale{
			Lang:    lang,
			Code:    code,
			Message: message,
		}).Error
	}
	return db.Model(&it).Updates(map[string]interface{}{
		"message": message,
		"version": it.Version + 1,
	}).Error
}

func T(db *gorm.DB, lang string, code string, args interface{}) string {
	val := fmt.Sprintf("%s.%s", lang, code)
	var it Locale
	if err := db.Where("lang = ? AND code = ?", lang, code).First(&it).Error; err != nil {
		return val
	}
	tpl, err := template.New("").Parse(it.Message)
	if err != nil {
		return val
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, args); err != nil {
		return val
	}
	return buf.String()
}
