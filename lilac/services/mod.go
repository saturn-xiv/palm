package services

import (
	"github.com/go-playground/validator/v10"
)

var (
	gl_validate *validator.Validate = validator.New()

	gl_email_validator_tag    = "required,email,lowercase,max=127"
	gl_code_validator_tag     = "required,alphanum,lowercase,min=2,max=31"
	gl_name_validator_tag     = "required,min=2,max=63"
	gl_password_validator_tag = "required,min=6,max=31"
	// gl_title_validator_tag    = "required,min=1,max=63"
	// gl_summary_validator_tag  = "required,min=2,max=511"
)

func init() {

}
