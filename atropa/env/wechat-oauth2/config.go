package wechat_oauth2

import (
	"net/http"
	"net/url"
)

// https://developers.weixin.qq.com/doc/oplatform/en/Website_App/WeChat_Login/Wechat_Login.html
type Config struct {
	AppID       string `toml:"app-id"`
	AppSecret   string `toml:"app-secret"`
	RedirectURI string `toml:"redirect-uri"`
}

func (p *Config) QrConnectURL(state string, lang string) string {
	query := url.Values{}
	query.Set("appid", p.AppID)
	query.Set("redirect_uri", p.RedirectURI)
	query.Set("response_type", "code")
	query.Set("scope", "snsapi_login")
	query.Set("state", state)
	query.Set("lang", lang)

	iu := url.URL{
		Scheme:   "https",
		Host:     "open.weixin.qq.com",
		Path:     "/connect/qrconnect",
		Fragment: "wechat_redirect",
		RawQuery: query.Encode(),
	}
	return iu.String()
}

func (p *Config) AccessToken(code string) (*AccessTokenResponse, error) {
	query := url.Values{}
	query.Set("appid", p.AppID)
	query.Set("secret", p.AppSecret)
	query.Set("code", code)
	query.Set("grant_type", "authorization_code")
	iu := url.URL{
		Scheme:   "https",
		Host:     "api.weixin.qq.com",
		Path:     "/sns/oauth2/access_token",
		RawQuery: query.Encode(),
	}

	res, err := http.Get(iu.String())
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var it AccessTokenResponse
	if err = CheckResponseError("wechat-oauth2 access-token", res, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

func (p *Config) RefreshToken(refresh_token string) (*RefreshTokenResponse, error) {
	query := url.Values{}
	query.Set("appid", p.AppID)
	query.Set("refresh_token", refresh_token)
	query.Set("grant_type", "refresh_token")
	iu := url.URL{
		Scheme:   "https",
		Host:     "api.weixin.qq.com",
		Path:     "/sns/oauth2/refresh_token",
		RawQuery: query.Encode(),
	}

	res, err := http.Get(iu.String())
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var it RefreshTokenResponse
	if err = CheckResponseError("wechat-oauth2 refresh-token", res, &it); err != nil {
		return nil, err
	}
	return &it, nil
}

// https://developers.weixin.qq.com/doc/oplatform/en/Website_App/WeChat_Login/Authorized_Interface_Calling_UnionID.html
func (p *Config) UserInfo(access_token string, open_id string, lang string) (*UserInfoResponse, error) {
	query := url.Values{}
	query.Set("access_token", access_token)
	query.Set("openid", open_id)
	query.Set("lang", lang)
	iu := url.URL{
		Scheme:   "https",
		Host:     "api.weixin.qq.com",
		Path:     "/sns/userinfo",
		RawQuery: query.Encode(),
	}

	res, err := http.Get(iu.String())
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	var it UserInfoResponse
	if err = CheckResponseError("wechat-oauth2 get user-info", res, &it); err != nil {
		return nil, err
	}
	return &it, nil
}
