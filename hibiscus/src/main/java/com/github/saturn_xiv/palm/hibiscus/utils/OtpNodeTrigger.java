package com.github.saturn_xiv.palm.hibiscus.utils;

import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.stereotype.Component;

import jakarta.annotation.Resource;

@Component("u.otp-node-trigger")
public class OtpNodeTrigger implements ApplicationRunner {

    @Override
    public void run(ApplicationArguments args) throws Exception {
        listener.startUp();
    }

    @Resource
    OtpNodeListener listener;
}
