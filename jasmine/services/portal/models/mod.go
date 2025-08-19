package models

import "github.com/go-playground/validator/v10"

var gl_validate = validator.New(validator.WithRequiredStructEnabled())
