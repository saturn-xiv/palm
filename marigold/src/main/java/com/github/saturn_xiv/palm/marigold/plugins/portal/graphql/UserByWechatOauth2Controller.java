package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.time.Duration;
import java.time.ZoneId;
import java.util.Locale;
import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.graphql.execution.ErrorType;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.etc.WechatProperties;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests.SignInByWechatOauth2Request;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.SignInResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserOauth2StateService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.marigold.wechat.WechatApiClient;
import com.github.saturn_xiv.palm.plugins.portal.v1.Oauth2State;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.Item.Type;
import com.google.protobuf.InvalidProtocolBufferException;

import graphql.GraphqlErrorException;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.user.by-wechat-oauth2")
public class UserByWechatOauth2Controller {
    // https://developers.weixin.qq.com/doc/oplatform/Website_App/WeChat_Login/Wechat_Login.html
    @QueryMapping
    public String signInByWechatOauth2Url(@Argument @NotBlank String client, @Argument @NotBlank String redirectUri,
            @Argument @NotBlank String lang) {
        final var config = wechatProperties.oauth2().get(client);
        final var state = userOauth2StateService.create(Oauth2State.Type.WECHAT);
        return config.signInUrl(Locale.forLanguageTag(lang), redirectUri, state);
    }

    @MutationMapping
    public SignInResponse signInByWechatOauth2(HttpServletRequest request_, @Argument @NotBlank String client,
            @Argument @Valid SignInByWechatOauth2Request request) throws InvalidProtocolBufferException {
        final var lang = Locale.forLanguageTag(request.lang());
        final var timezone = ZoneId.of(request.timezone());
        final var config = wechatProperties.oauth2().get(client);

        final var state = userOauth2StateService.get(request.state());
        if (state.getType() != Oauth2State.Type.WECHAT) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN).build();
        }
        if (state.hasUser()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("Invalid state").build();
        }

        logger.debug("fetch access token for ({}, {})", request.code(), config.appId());
        final var accessToken = wechatApiClient.accessTokenByCode(config.appId(), config.secret(), request.code(),
                Optional.empty());
        accessToken.check();
        this.redisTemplate.opsForValue().set(this.accessTokenKey(accessToken.getAccessToken()),
                accessToken.getUnionId(), Duration.ofSeconds(accessToken.getExpiresIn() - 5));
        final var info = wechatApiClient.userInfo(accessToken.getAccessToken(), config.appId(),
                "zh".equals(lang.getLanguage()) ? WechatApiClient.SIMPLIFIED_CHINESE : WechatApiClient.ENGLISH);
        info.check();

        final var ip = request_.getRemoteHost();

        userService.signIn(config.appId(), accessToken, info, ip, lang, timezone);
        var it = wechatOauth2UserRepository.findByAppIdAndOpenId(config.appId(), accessToken.getOpenId()).get();
        if (!it.isAvailable()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN).build();
        }
        return new SignInResponse(it.getUser(), it.getSn(), Type.WECHAT_OAUTH2);
    }

    private String accessTokenKey(String s) {
        return String.format("wechat-oauth2.access-token.%s", s);
    }

    @Resource
    WechatApiClient wechatApiClient;
    @Resource
    UserService userService;
    @Resource
    UserRepository userRepository;
    @Resource
    WechatOauth2UserRepository wechatOauth2UserRepository;
    @Resource
    WechatProperties wechatProperties;
    @Resource
    UserOauth2StateService userOauth2StateService;
    @Autowired
    StringRedisTemplate redisTemplate;

    private final static Logger logger = LoggerFactory.getLogger(UserByWechatOauth2Controller.class);
}
