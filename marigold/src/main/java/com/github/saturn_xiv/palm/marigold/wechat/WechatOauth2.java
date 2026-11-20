package com.github.saturn_xiv.palm.marigold.wechat;

import java.io.Serializable;
import java.util.Locale;

public record WechatOauth2(String appId, String secret) implements Serializable {

    public String signInUrl(Locale locale, String home, String state) {
        final var lang = "zh".equals(locale.getLanguage()) ? "cn" : "en";
        return String.format(
                "https://open.weixin.qq.com/connect/qrconnect?lang=%sappid=%s&redirect_uri=%s/my/anonymous/sign-in-by-wechat-oauth2&response_type=code&scope=snsapi_login&state=%s#wechat_redirect",
                lang, appId, home, state);
    }

}
