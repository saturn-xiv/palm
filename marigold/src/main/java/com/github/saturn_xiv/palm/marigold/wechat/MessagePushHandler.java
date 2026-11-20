package com.github.saturn_xiv.palm.marigold.wechat;

import java.util.Optional;

import com.fasterxml.jackson.databind.ObjectMapper;

public interface MessagePushHandler<T, R> {
    Optional<T> parse(ObjectMapper objectMapper, String message);

    R execute(String appId, T request, String ip);
}
