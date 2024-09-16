package google_oauth2

// https://developers.google.com/identity/openid-connect/openid-connect#obtainuserinfo
type IdToken struct {
	Subject       string  `json:"sub"`
	Picture       *string `json:"picture,omitempty"`
	Locale        *string `json:"locale,omitempty"`
	Name          *string `json:"name,omitempty"`
	FamilyName    *string `json:"family_name,omitempty"`
	GivenName     *string `json:"given_name,omitempty"`
	Email         *string `json:"email,omitempty"`
	Profile       *string `json:"profile,omitempty"`
	EmailVerified bool    `json:"email_verified"`
}
