package com.github.saturn_xiv.palm.marigold.plugins.portal.queues;

import org.springframework.stereotype.Component;

import com.google.protobuf.Message;

import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.core.MessageBuilder;
import org.springframework.amqp.core.MessageDeliveryMode;
import org.springframework.amqp.rabbit.core.RabbitTemplate;

@Component("q.message-sender")
public class MessageSender {
    public void produce(String queue, String messageId, Message message) {
        this.send("", queue, UUID.randomUUID().toString(), PROTOBUF_CONTENT_TYPE, message.toByteArray());
    }

    private void send(String exchange, String routingKey, String messageId, String contentType, byte[] body) {
        logger.info("send message({} bytes) {}://{} to ({},{})", body.length, contentType, messageId, exchange,
                routingKey);
        final var message = MessageBuilder.withBody(body).setMessageId(messageId).setContentType(contentType)
                .setDeliveryMode(MessageDeliveryMode.PERSISTENT).build();
        rabbitTemplate.send(exchange, routingKey, message);
    }

    // https://protobuf.dev/reference/protobuf/mime-types/
    public final static String PROTOBUF_CONTENT_TYPE = "application/protobuf";

    RabbitTemplate rabbitTemplate;
    private static final Logger logger = LoggerFactory.getLogger(MessageSender.class);
}
