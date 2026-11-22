package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.io.IOException;
import java.time.ZoneId;
import java.util.Locale;
import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.graphql.data.method.annotation.Argument;
import org.springframework.graphql.data.method.annotation.MutationMapping;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.graphql.execution.ErrorType;
import org.springframework.web.bind.annotation.ControllerAdvice;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.etc.GoogleProperties;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests.SignInByGoogleOauth2Request;
import com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses.SignInResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.GoogleOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserOauth2StateService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.plugins.portal.v1.Oauth2State;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse.Item.Type;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.services.oauth2.Oauth2;

import graphql.GraphqlErrorException;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;
import jakarta.validation.constraints.NotBlank;

@ControllerAdvice("portal.g.user.by-google-oauth2")
public class UserByGoogleOauth2Controller {
    // https://developers.google.com/identity/protocols/oauth2/web-server
    // https://docs.cloud.google.com/java/docs/reference/google-auth-library/latest/overview.html
    // https://developers.google.com/api-client-library/java/google-oauth-java-client/oauth2
    @QueryMapping
    public String signInByGoogleOauth2Url(@Argument @NotBlank String client, @Argument @NotBlank String redirectUri)
            throws IOException {
        final var config = googleProperties.oauth2().get(client);
        final var state = userOauth2StateService.create(Oauth2State.Type.GOOGLE);
        return config.signInUrl(redirectUri, state);
    }

    // https://developers.google.com/identity/protocols/oauth2/web-server#tokenrevoke
    public Succeed revokeGoogleOauth2() {
        // TODO
        return new Succeed();
    }

    @MutationMapping
    public SignInResponse signInByGoogleOauth2(HttpServletRequest request_, @Argument @NotBlank String client,
            @Argument @Valid SignInByGoogleOauth2Request request) throws IOException {
        final var lang = Locale.forLanguageTag(request.lang());
        final var timezone = ZoneId.of(request.timezone());
        final var config = googleProperties.oauth2().get(client);

        final var state = userOauth2StateService.get(request.state());
        if (state.getType() != Oauth2State.Type.GOOGLE) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN).build();
        }
        if (state.hasUser()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN)
                    .message("Invalid state").build();
        }

        logger.debug("fetch access token for ({}, {})", request.code(), config.clientId());

        final String user = UUID.randomUUID().toString();
        final var reply = config.exchange(request.code(), request.redirectUri(), user);
        var oauth2 = new Oauth2.Builder(
                new NetHttpTransport(),
                GsonFactory.getDefaultInstance(),
                reply.getRight())
                .setApplicationName(applicationName)
                .build();
        var userInfo = oauth2.userinfo().get().execute();

        final var ip = request_.getRemoteHost();
        userService.signIn(config.clientId(), reply.getLeft(), userInfo, ip, lang, timezone);
        var it = googleOauth2UserRepository.findByClientIdAndSub(config.clientId(), reply.getLeft().getAccessToken())
                .get();
        if (!it.isAvailable()) {
            throw GraphqlErrorException.newErrorException().errorClassification(ErrorType.FORBIDDEN).build();
        }
        return new SignInResponse(it.getUser(), it.getSn(), Type.WECHAT_OAUTH2);
    }

    @Resource
    UserService userService;
    @Resource
    GoogleOauth2UserRepository googleOauth2UserRepository;
    @Autowired
    StringRedisTemplate redisTemplate;
    @Resource
    GoogleProperties googleProperties;
    @Resource
    UserOauth2StateService userOauth2StateService;
    @Value("${spring.application.name}")
    String applicationName;

    private final static Logger logger = LoggerFactory.getLogger(UserByGoogleOauth2Controller.class);
}
