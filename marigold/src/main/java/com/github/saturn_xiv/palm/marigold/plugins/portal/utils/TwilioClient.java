package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import com.twilio.Twilio;
import com.twilio.rest.api.v2010.account.Message;
import com.twilio.type.PhoneNumber;

import jakarta.annotation.PostConstruct;

// https://www.twilio.com/docs/glossary/what-is-a-webhook
@Component("portal.u.twilio")
public class TwilioClient {

    // https://www.twilio.com/docs/messaging/tutorials/how-to-send-sms-messages#send-an-sms-message
    // https://www.twilio.com/docs/glossary/what-e164
    public void sendSms(String from, String to, String message, Optional<String> callback) {
        logger.info("send sms message to ({}) from {}", to, from);
        var creator = Message.creator(new PhoneNumber(to), new PhoneNumber(from), message);
        if (!callback.isEmpty()) {
            creator = creator.setStatusCallback(callback.get());
        }
        final var msg = creator.create();
        logger.info("Message SID {}", msg.getSid());
    }

    @PostConstruct
    void init() {
        Twilio.init(apiKey, apiSecret, accountSid);

    }

    @Value("${palm.twilio.account-sid}")
    String accountSid;
    @Value("${palm.twilio.api-key}")
    String apiKey;
    @Value("${palm.twilio.api-secret}")
    String apiSecret;

    private static final Logger logger = LoggerFactory.getLogger(TwilioClient.class);
}
