package accounting

import (
	"gorm.io/gorm"

	"github.com/saturn-xiv/palm/lilac/models"
)

// https://www.gnucash.org/docs.phtml

type Account struct {
	models.Model `gorm:"embedded"`

	UserID     uint32 `gorm:"uniqueIndex;not null"`
	Email      string `gorm:"uniqueIndex;not null;size:127"`
	Name       string `gorm:"index;not null;size:63"`
	GivenName  string `gorm:"index;not null;size:31"`
	FamilyName string `gorm:"index;not null;size:31"`
	Gender     string `gorm:"index;not null;size:15"`
	Picture    string `gorm:"not null;size:255"`
	Locale     string `gorm:"index;not null;size:15"`
	Sub        string `gorm:"uniqueIndex;not null;size:127"`
	Code       string `gorm:"not null"`
	Token      string `gorm:"not null;size:127"`
}

type Asset struct {
	models.Model `gorm:"embedded"`
}

func AutoMigrate(db *gorm.DB) error {
	return db.AutoMigrate(
		&Account{},
		&Asset{},
	)
}
