package wechat_mini_program

type Code2SessionResponse struct {
	SessionKey string  `json:"session_key"`
	UnionID    *string `json:"unionid,omitempty"`
	OpenID     string  `json:"openid"`
}

type AccessTokenResponse struct {
	AccessToken string `json:"access_token"`
	ExpiresIn   uint   `json:"expires_in"`
}
