package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import java.time.Duration;
import java.util.Base64;
import java.util.Optional;
import java.util.UUID;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.plugins.portal.v1.Oauth2State;
import com.google.protobuf.InvalidProtocolBufferException;

@Service("portal.s.user-oauth2-state")
public class UserOauth2StateService {

    public String create(Oauth2State.Type type) {
        return this.create(type, Optional.empty());
    }

    public String create(Oauth2State.Type type, Item user) {
        return this.create(type, Optional.of(user.getSn()));
    }

    private String create(Oauth2State.Type type, Optional<String> user) {
        var builder = Oauth2State.newBuilder().setType(type);
        if (user.isPresent()) {
            builder.setUser(user.get());
        }
        var it = builder.build();
        final var key = UUID.randomUUID().toString();
        var buf = it.toByteArray();
        var val = Base64.getUrlEncoder().withoutPadding().encodeToString(buf);
        redisTemplate.opsForValue().set(this.key(key), val, Duration.ofMinutes(5));
        return key;
    }

    public Oauth2State get(String key) throws InvalidProtocolBufferException {
        final var val = redisTemplate.opsForValue().get(this.key(key));
        var buf = Base64.getUrlDecoder().decode(val);
        return Oauth2State.parseFrom(buf);
    }

    private String key(String s) {
        return String.format("users.oauth2.state://%s", s);
    }

    @Autowired
    StringRedisTemplate redisTemplate;

}
