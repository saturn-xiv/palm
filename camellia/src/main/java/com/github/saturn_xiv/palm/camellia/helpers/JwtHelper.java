package com.github.saturn_xiv.palm.camellia.helpers;

import jakarta.annotation.PostConstruct;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

@Component("palm.camellia.jwt-helper")
public class JwtHelper {

    @PostConstruct
    void init() {

    }

    private static final Logger logger = LoggerFactory.getLogger(JwtHelper.class);
}
