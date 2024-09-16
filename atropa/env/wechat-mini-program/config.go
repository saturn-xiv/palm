package wechat_mini_program

import (
	"net/http"
	"net/url"

	wechat_oauth2 "github.com/saturn-xiv/palm/atropa/env/wechat-oauth2"
)

// https://developers.weixin.qq.com/doc/oplatform/en/Miniprogram_Frame/
// https://developers.weixin.qq.com/miniprogram/en/dev/framework/open-ability/login.html
type Config struct {
	AppID     string `toml:"app-id"`
	AppSecret string `toml:"app-secret"`
}

// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/mp-access-token/getAccessToken.html
func (p *Config) GetAccessToken() (*AccessTokenResponse, error) {
	query := url.Values{}
	query.Set("appid", p.AppID)
	query.Set("secret", p.AppSecret)
	query.Set("grant_type", "client_credential")
	iu := url.URL{
		Scheme:   "https",
		Host:     "api.weixin.qq.com",
		Path:     "/cgi-bin/token",
		RawQuery: query.Encode(),
	}

	res, err := http.Get(iu.String())
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var it AccessTokenResponse
	if err = wechat_oauth2.CheckResponseError("wechat-mini-program get access-token", res, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

// https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/user-login/code2Session.html
func (p *Config) Code2Session(code string) (*Code2SessionResponse, error) {
	query := url.Values{}
	query.Set("appid", p.AppID)
	query.Set("secret", p.AppSecret)
	query.Set("js_code", code)
	query.Set("grant_type", "authorization_code")
	iu := url.URL{
		Scheme:   "https",
		Host:     "api.weixin.qq.com",
		Path:     "/sns/jscode2session",
		RawQuery: query.Encode(),
	}

	res, err := http.Get(iu.String())
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var it Code2SessionResponse
	if err = wechat_oauth2.CheckResponseError("wechat-mini-program code-to-session", res, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
