package models

// https://developers.google.com/identity/protocols/oauth2
// https://developers.google.com/identity/protocols/oauth2/web-server
// https://github.com/googleapis/google-api-go-client/tree/main
type GoogleUser struct {
	Model `gorm:"embedded"`

	UserID     uint32   `gorm:"uniqueIndex;not null"`
	Email      string `gorm:"uniqueIndex;not null;size:127"`
	GivenName  string `gorm:"index;not null;size:31"`
	FamilyName string `gorm:"index;not null;size:31"`
	Picture    string `gorm:"not null;size:255"`
	Locale     string `gorm:"index;not null;size:15"`
	Sub        string `gorm:"uniqueIndex;not null;size:127"`
	Code       string `gorm:"not null"`
	Token      string `gorm:"not null;size:127"`
}
