package com.github.saturn_xiv.palm.marigold.wechat.impl;

import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;
import com.github.saturn_xiv.palm.marigold.wechat.MessagePushHandler;
import com.github.saturn_xiv.palm.marigold.wechat.requests.AuthorizationChangeRequest;

import jakarta.annotation.Resource;

@Component("portal.u.message-push-handler")
public class AuthorizationChangeMessagePushHandler implements MessagePushHandler<AuthorizationChangeRequest, Succeed> {

    @Override
    public Optional<AuthorizationChangeRequest> parse(ObjectMapper objectMapper, String message) {
        try {
            final var request = objectMapper.readValue(message, AuthorizationChangeRequest.class);
            return Optional.of(request);
        } catch (JsonProcessingException ex) {

        }
        return Optional.empty();
    }

    @Override
    public Succeed execute(String appId, AuthorizationChangeRequest request, String ip) {
        if (request.isRevoke()) {
            logger.info("revoke user {}<{}>", request.openId(), appId);
            userService.destroyWechatOauth2User(appId, request.openId(), ip, "Revoke by self");
            return new Succeed();
        }
        logger.info("update nickname {}<{}> {}=>{}", request.openId(), appId, request.fromUserName(),
                request.toUserName());
        userService.setWechatOauth2UseNickname(appId, request.openId(), request.toUserName());
        return new Succeed();
    }

    @Resource
    UserService userService;

    private final static Logger logger = LoggerFactory.getLogger(AuthorizationChangeMessagePushHandler.class);
}
