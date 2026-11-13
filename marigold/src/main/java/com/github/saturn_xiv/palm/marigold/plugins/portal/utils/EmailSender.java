package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Component;
import org.springframework.util.StopWatch;

import jakarta.annotation.Resource;
import jakarta.mail.MessagingException;
import jakarta.mail.internet.MimeMessage;

@Component("portal.u.email-sender")
public class EmailSender {

    public MimeMessage createMimeMessage() {
        return this.mailSender.createMimeMessage();
    }

    @Async
    public void send(MimeMessage message) throws MessagingException {
        logger.info("Send email({},{}): {}", this.from, message.getMessageID(), message.getSubject());
        var watch = new StopWatch();
        watch.start();
        mailSender.send(message);
        watch.stop();
        logger.info("executed in {} ms", watch.prettyPrint());
    }

    @Resource
    JavaMailSender mailSender;
    @Value("${spring.mail.username}")
    String from;
    private static final Logger logger = LoggerFactory.getLogger(EmailSender.class);
}
