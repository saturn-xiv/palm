package v2

func (p *WechatOauth2UserIndexResponse_Item_Lang) ToString() string {
	if *p == WechatOauth2UserIndexResponse_Item_En {
		return "en"
	}
	return "cn"
}
