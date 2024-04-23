package models

import (
	"errors"
	"fmt"
	"time"

	"gorm.io/gorm"

	pb "github.com/saturn-xiv/palm/lilac/auth/v2"
)

type Locale struct {
	ID        uint32    `gorm:"primaryKey"`
	Lang      string    `gorm:"index;index:,unique,composite:lang_code;not null;size:15"`
	Code      string    `gorm:"index;index:,unique,composite:lang_code;not null;size:255"`
	Message   string    `gorm:"not null"`
	Version   uint32    `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}

func (p *Locale) Key() string {
	return fmt.Sprintf("%s.%s", p.Lang, p.Code)
}

func GetLocaleByLangAndCode(db *gorm.DB, lang string, code string) (*Locale, error) {
	var it Locale
	if rst := db.Where(&Locale{Code: code, Lang: lang}).First(&it); rst.Error != nil {
		return nil, rst.Error
	}
	return &it, nil
}

func GetLocaleByLang(db *gorm.DB, lang string) ([]Locale, error) {
	var items []Locale
	if rst := db.Order("code asc").Find(&items); rst.Error != nil {
		return nil, rst.Error
	}
	return items, nil
}

func GetLocaleByPager(db *gorm.DB, pager *pb.Pager) ([]Locale, *pb.Pagination, error) {
	var total int64
	if rst := db.Model(&Locale{}).Count(&total); rst.Error != nil {
		return nil, nil, rst.Error
	}

	var items []Locale
	if rst := db.Order("updated_at desc").Offset(pager.Offset(total)).Limit(pager.Size_()).Find(&items); rst.Error != nil {
		return nil, nil, rst.Error
	}
	return items, pb.NewPagination(pager, total), nil
}

func SetLocale(db *gorm.DB, lang string, code string, message string) error {
	now := time.Now()
	var it Locale
	rst := db.Where(&Locale{Code: code, Lang: lang}).First(&it)
	if errors.Is(rst.Error, gorm.ErrRecordNotFound) {
		return db.Create(&Locale{
			Code:      code,
			Lang:      lang,
			Message:   message,
			UpdatedAt: now,
			CreatedAt: now,
		}).Error
	}
	if rst.Error == nil {
		it.Message = message
		it.UpdatedAt = now
		it.Version += 1
		return db.Save(&it).Error
	}
	return rst.Error
}
