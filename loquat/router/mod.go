package router

import (
	"gorm.io/gorm"

	v2 "github.com/saturn-xiv/palm/loquat/router/v2"
)

func Export(db *gorm.DB) (*v2.Router, error) {
	var rt v2.Router
	// TODO
	return &rt, nil
}

func Import(db *gorm.DB, rt *v2.Router) error {
	// TODO
	return nil
}
