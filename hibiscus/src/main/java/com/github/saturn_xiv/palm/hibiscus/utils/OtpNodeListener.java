package com.github.saturn_xiv.palm.hibiscus.utils;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Component;

import jakarta.annotation.Resource;

@Component("u.otp-node-listener")
public class OtpNodeListener {
    @Async
    public void startUp() {
        while (!Thread.currentThread().isInterrupted()) {
            try {
                logger.debug("running");
                Thread.sleep(5000);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    @Resource
    CasbinEnforcer casbinEnforcer;

    private static final Logger logger = LoggerFactory.getLogger(OtpNodeListener.class);
}
