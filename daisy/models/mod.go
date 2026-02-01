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
	ID        uint      `gorm:"primarykey"`
	CreatedAt time.Time `gorm:"not null"`
	UpdatedAt time.Time `gorm:"not null"`
	DeletedAt gorm.DeletedAt
}
