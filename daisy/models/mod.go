package models

import (
	"strings"
	"time"

	"github.com/go-playground/validator/v10"
	"gorm.io/gorm"
)

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

func ToCode(s string) string {
	return strings.TrimSpace(strings.ToLower(s))
}

type Model struct {
	ID        uint `gorm:"primarykey"`
	DeletedAt gorm.DeletedAt
	Version   uint      `gorm:"not null;default:0"`
	UpdatedAt time.Time `gorm:"not null"`
	CreatedAt time.Time `gorm:"not null"`
}
