package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.time.OffsetDateTime;
import java.time.ZoneOffset;
import java.util.Base64;

import org.springframework.boot.info.BuildProperties;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.plugins.portal.v1.PasswordBox;
import com.google.protobuf.InvalidProtocolBufferException;

import jakarta.annotation.Resource;

@Controller("portal.g.home")
public class HomeController {
    @QueryMapping
    public String apiVersion() {
        return buildProperties.getVersion();
    }

    @QueryMapping
    public OffsetDateTime buildTime() {
        return buildProperties.getTime().atOffset(ZoneOffset.UTC);
    }

    public static String password(String raw) throws InvalidProtocolBufferException {
        final var buf = Base64.getUrlDecoder().decode(raw);
        final var box = PasswordBox.parseFrom(buf);
        return box.getPlain();
    }

    public static String email(String raw) {
        return raw.trim().toLowerCase();
    }

    @Resource
    BuildProperties buildProperties;
}
