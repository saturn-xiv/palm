package com.github.saturn_xiv.palm.hibiscus.otp_node;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpNode;
import com.github.saturn_xiv.palm.hibiscus.otp_node.handlers.Argon2Handler;
import com.github.saturn_xiv.palm.hibiscus.otp_node.handlers.RbacHandler;
import com.github.saturn_xiv.palm.hibiscus.otp_node.handlers.StatusHandler;

import jakarta.annotation.Resource;

@Component("otp-node.trigger")
public class Trigger implements ApplicationRunner {

    @Override
    public void run(ApplicationArguments args) throws Exception {
        logger.info("Launch a Java otp node {}", name);
        OtpNode node = new OtpNode(name, cookie);
        listener.startUp(node, statusHandler);
        listener.startUp(node, rbacHandler);
        listener.startUp(node, argon2Handler);
    }

    @Resource
    Listener listener;
    @Resource
    StatusHandler statusHandler;
    @Resource
    Argon2Handler argon2Handler;
    @Resource
    RbacHandler rbacHandler;
    @Value("${hibiscus.otp-node.name}")
    String name;
    @Value("${hibiscus.otp-node.cookie}")
    String cookie;

    private static final Logger logger = LoggerFactory.getLogger(Trigger.class);
}
