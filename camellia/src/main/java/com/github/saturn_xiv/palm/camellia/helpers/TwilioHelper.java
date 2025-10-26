package com.github.saturn_xiv.palm.camellia.helpers;

import jakarta.annotation.PostConstruct;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

@Component("palm.camellia.twilio-helper")
public class TwilioHelper {

    @PostConstruct
    void init() {

    }

    private static final Logger logger = LoggerFactory.getLogger(TwilioHelper.class);
}
