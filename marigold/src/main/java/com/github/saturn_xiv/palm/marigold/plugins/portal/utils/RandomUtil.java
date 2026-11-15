package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.security.SecureRandom;

import org.apache.commons.lang3.RandomStringUtils;
import org.springframework.stereotype.Component;

import jakarta.annotation.PostConstruct;

@Component("portal.u.random")
public class RandomUtil {

    public String alphanumeric(int c) {
        return RandomStringUtils.secure().nextAlphanumeric(c);
    }

    public byte[] bytes(int c) {
        byte[] buf = new byte[c];
        secureRandom.nextBytes(buf);
        return buf;
    }

    @PostConstruct
    void init() {
        this.secureRandom = new SecureRandom();
    }

    private SecureRandom secureRandom;
}
