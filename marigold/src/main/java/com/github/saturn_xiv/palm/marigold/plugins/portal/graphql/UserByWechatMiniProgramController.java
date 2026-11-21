package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.time.ZoneId;
import java.util.Locale;
import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.etc.WechatProperties;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests.SignInByWechatMiniProgramRequest;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.SignInResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatMiniProgramUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.marigold.wechat.WechatApiClient;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.Item.Type;

import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;

@Controller("portal.g.user.by-wechat-mini-program")
public class UserByWechatMiniProgramController {
    // https://developers.weixin.qq.com/miniprogram/dev/framework/open-ability/login.html
    @MutationMapping
    public SignInResponse signInByWechatMiniProgram(HttpServletRequest request_, @Argument @NotBlank String client,
            @Argument @Valid SignInByWechatMiniProgramRequest request) {
        final var lang = Locale.forLanguageTag(request.lang());
        final var timezone = ZoneId.of(request.timezone());
        final var config = wechatProperties.miniProgram().get(client);
        logger.info("fetch session-key for {}@{}", request.code(), config.appId());
        final var session = wechatApiClient.jscode2session(config.appId(), config.secret(), request.code(),
                Optional.empty());
        session.check();
        final var ip = request_.getRemoteHost();
        userService.signIn(config.appId(), session, ip, lang, timezone);

        final var it = wechatMiniProgramUserRepository.findByAppIdAndOpenId(config.appId(), session.getOpenId()).get();
        return new SignInResponse(it.getUser(), it.getSn(), Type.WECHAT_MINI_PROGRAM);
    }

    public Succeed setInfoForWechatMiniProgramUser(@Argument @NotBlank String nickname,
            @Argument @NotBlank String avatarUrl) {
        // TODO
        return new Succeed();
    }

    @Resource
    WechatApiClient wechatApiClient;
    @Resource
    UserService userService;
    @Resource
    WechatProperties wechatProperties;
    @Resource
    WechatMiniProgramUserRepository wechatMiniProgramUserRepository;

    private final static Logger logger = LoggerFactory.getLogger(UserByWechatMiniProgramController.class);
}
