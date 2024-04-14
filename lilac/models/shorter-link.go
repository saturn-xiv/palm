package models

import (
	"encoding/base64"
	"encoding/binary"
	"time"

	"gorm.io/gorm"
)

type ShorterLink struct {
	ID        uint32    `gorm:"primaryKey"`
	Url       string    `gorm:"uniqueIndex;not null;size:255"`
	Summary   string    `gorm:"index;not null;size:511"`
	CreatedAt time.Time `gorm:"not null"`
}

func (p *ShorterLink) Code() string {
	buf := make([]byte, 4)
	binary.LittleEndian.PutUint32(buf, p.ID)
	return base64.URLEncoding.WithPadding(base64.NoPadding).EncodeToString(buf)
}

func GetShorterLinkByCode(db *gorm.DB, code string) (*ShorterLink, error) {
	buf, err := base64.URLEncoding.WithPadding(base64.NoPadding).DecodeString(code)
	if err != nil {
		return nil, err
	}
	id := binary.LittleEndian.Uint32(buf)
	var it ShorterLink
	if rst := db.First(&it, id); rst.Error != nil {
		return nil, err
	}
	return &it, nil
}
