package models

import (
	"bytes"
	"errors"
	"log/slog"
	"text/template"
	"time"

	"gorm.io/gorm"
)

type Locale struct {
	ID        uint      `gorm:"primarykey"`
	Lang      string    `gorm:"uniqueIndex:idx_lang_code;index;not null;size:15"`
	Code      string    `gorm:"uniqueIndex:idx_lang_code;index;not null;size:255"`
	Message   string    `gorm:"not null;type:text"`
	Version   uint      `gorm:"not null;default:0"`
	CreatedAt time.Time `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
}

func (Locale) TableName() string {
	return "locales"
}

func T(db *gorm.DB, lang string, code string, args map[string]interface{}) string {
	it, err := tr(db, lang, code, args)
	if err != nil {
		return code
	}
	return it
}

func CountLocale(db *gorm.DB) (int64, error) {
	var c int64
	if err := db.Model(&Locale{}).Count(&c).Error; err != nil {
		return 0, err
	}
	return c, nil
}

func SetLocale(db *gorm.DB, lang string, code string, message string) error {
	it, err := LocaleByLangAndCode(db, lang, code)
	if err == nil {
		if err = db.Model(&it).Updates(map[string]interface{}{
			"message": message,
			"version": it.Version + 1,
		}).Error; err != nil {
			return err
		}
	}
	if !errors.Is(err, gorm.ErrRecordNotFound) {
		return err
	}

	if err := db.Create(&Locale{
		Lang:    lang,
		Code:    code,
		Message: message,
	}).Error; err != nil {
		return err
	}

	return nil
}

func LocaleByLangAndCode(db *gorm.DB, lang string, code string) (*Locale, error) {
	var it Locale
	if err := db.Where(map[string]interface{}{"lang": lang, "code": code}).First(&it).Error; err != nil {
		slog.Error(err.Error())
		return nil, err
	}
	return &it, nil
}

func tr(db *gorm.DB, lang string, code string, args map[string]interface{}) (string, error) {
	it, err := LocaleByLangAndCode(db, lang, code)
	if err != nil {
		return "", err
	}
	if args == nil {
		return it.Message, nil
	}
	tpl, err := template.New("").Parse(it.Message)
	if err != nil {
		return "", err
	}
	var buf bytes.Buffer
	if err = tpl.Execute(&buf, args); err != nil {
		return "", err
	}
	return buf.String(), nil
}
