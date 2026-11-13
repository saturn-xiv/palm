package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.util.Properties;

import org.springframework.data.redis.connection.RedisConnection;
import org.springframework.data.redis.connection.RedisConnectionFactory;
import org.springframework.stereotype.Component;

import jakarta.annotation.Resource;

@Component("portal.u.redis-client")
public class RedisClient {
    public String version() {
        try (RedisConnection connection = connectionFactory.getConnection()) {
            Properties info = connection.serverCommands().info("server");
            return info == null ? null : info.getProperty("redis_version");
        }
    }

    @Resource
    RedisConnectionFactory connectionFactory;
}
