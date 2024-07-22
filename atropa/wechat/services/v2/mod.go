package v2

func (p *Oauth2QrConnectUrlRequest_Lang) ToString() string {
	if *p == Oauth2QrConnectUrlRequest_En {
		return "en"
	}
	return "cn"
}
