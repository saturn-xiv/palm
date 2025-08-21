package models

import (
	"time"

	portal_v2 "github.com/saturn-xiv/palm/jasmine/services/portal/v2"
	"gorm.io/gorm"
)

const (
	LOCALHOST = "127.0.0.1"
)

type Log struct {
	ID        uint32 `gorm:"primaryKey"`
	CreatedAt time.Time

	UserID       uint32
	Plugin       string
	IP           string
	Level        int32
	ResourceType string
	ResourceID   *uint32
	Message      string
}

func (Log) TableName() string {
	return "logs"
}

type CreateLogForm struct {
	Plugin       string `validate:"required,min=2,max=31"`
	IP           string `validate:"required,min=2,max=45"`
	ResourceType string `validate:"required,min=2,max=127"`
	Message      string `validate:"required,min=2"`
}

func (p *CreateLogForm) Execute(db *gorm.DB, user uint32, level portal_v2.UserLogsResponse_Item_Level, resource_id *uint32) error {
	if err := gl_validate.Struct(p); err != nil {
		return err
	}
	return db.Create(&Log{
		UserID:       user,
		Plugin:       p.Plugin,
		IP:           p.IP,
		Level:        int32(level),
		ResourceType: p.ResourceType,
		ResourceID:   resource_id,
		Message:      p.Message},
	).Error
}
