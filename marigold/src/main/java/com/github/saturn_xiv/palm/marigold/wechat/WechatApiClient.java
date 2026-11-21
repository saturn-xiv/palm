package com.github.saturn_xiv.palm.marigold.wechat;

import java.util.Optional;

import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.service.annotation.GetExchange;
import org.springframework.web.service.annotation.HttpExchange;

import com.github.saturn_xiv.palm.marigold.wechat.responses.AccessTokenByCodeResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.ErrorResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.JsCode2SessionResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.RefreshAccessTokenResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.UserInfoResponse;

@HttpExchange(url = "https://api.weixin.qq.com")
public interface WechatApiClient {
        @GetExchange("/wxa/checksession")
        ErrorResponse checkSession(@RequestParam(name = "access_token") String accessToken,
                        @RequestParam(name = "openid") String openId,
                        @RequestParam String signature,
                        @RequestParam(name = "sig_method", defaultValue = "hmac_sha256") Optional<String> signMethod);

        @GetExchange("/sns/jscode2session")
        JsCode2SessionResponse jscode2session(@RequestParam(name = "appid") String appId, @RequestParam String secret,
                        @RequestParam(name = "js_code") String jsCode,
                        @RequestParam(name = "grant_type", defaultValue = "authorization_code") Optional<String> grantType);

        @GetExchange("/sns/oauth2/access_token")
        AccessTokenByCodeResponse accessTokenByCode(@RequestParam(name = "appid") String appId,
                        @RequestParam String secret,
                        @RequestParam String code,
                        @RequestParam(name = "grant_type", defaultValue = "authorization_code") Optional<String> grantType);

        @GetExchange("/sns/oauth2/access_token")
        RefreshAccessTokenResponse refreshAccessToken(@RequestParam(name = "appid") String appId,
                        @RequestParam String secret,
                        @RequestParam(name = "refresh_token") String refreshToken,
                        @RequestParam(name = "grant_type", defaultValue = "refresh_token") Optional<String> grantType);

        @GetExchange("/sns/oauth2/access_token")
        ErrorResponse authRefreshAccess(
                        @RequestParam(name = "access_token") String accessToken,
                        @RequestParam(name = "openid") String openId);

        @GetExchange("/sns/userinfo")
        UserInfoResponse userInfo(
                        @RequestParam(name = "access_token") String accessToken,
                        @RequestParam(name = "openid") String openId,
                        @RequestParam(defaultValue = ENGLISH) String lang);

        public final static String ENGLISH = "en";
        public final static String SIMPLIFIED_CHINESE = "zh_CN";
        public final static String TRADITIONAL_CHINESE = "zh_TW";
}
