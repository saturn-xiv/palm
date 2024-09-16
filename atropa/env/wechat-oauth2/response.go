package wechat_oauth2

type AccessTokenResponse struct {
	AccessToken  string  `json:"access_token"`
	ExpiresIn    uint64  `json:"expires_in"`
	RefreshToken string  `json:"refresh_token"`
	OpenID       string  `json:"openid"`
	Scope        string  `json:"scope"`
	UnionID      *string `json:"unionid"`
}

type RefreshTokenResponse struct {
	AccessToken  string `json:"access_token"`
	ExpiresIn    uint64 `json:"expires_in"`
	RefreshToken string `json:"refresh_token"`
	OpenID       string `json:"openid"`
	Scope        string `json:"scope"`
}

type UserInfoResponse struct {
	OpenID     string   `json:"openid"`
	Nickname   string   `json:"nickname"`
	Sex        uint32   `json:"sex"`
	Province   string   `json:"province"`
	City       string   `json:"city"`
	Country    string   `json:"country"`
	HeadImgURL *string  `json:"headimgurl,omitempty"`
	Privilege  []string `json:"privilege"`
	UnionID    string   `json:"unionid"`
}
