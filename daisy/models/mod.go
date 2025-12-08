package models

import (
	"strings"

	"github.com/go-playground/validator/v10"
)

var gl_validate = validator.New(validator.WithRequiredStructEnabled())

func ToCode(s string) string {
	return strings.TrimSpace(strings.ToLower(s))
}
