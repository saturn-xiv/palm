package com.github.saturn_xiv.palm.camellia.helpers;

import jakarta.annotation.PostConstruct;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;


@Component("palm.camellia.s3-helper")
public class S3Helper {

    @PostConstruct
    void init() {

    }

    private static final Logger logger = LoggerFactory.getLogger(S3Helper.class);
}
