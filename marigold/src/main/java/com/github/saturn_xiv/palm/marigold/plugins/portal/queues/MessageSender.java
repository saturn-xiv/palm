package com.github.saturn_xiv.palm.marigold.plugins.portal.queues;

import org.springframework.stereotype.Component;

import com.google.protobuf.Message;

import jakarta.annotation.Nullable;
import jakarta.annotation.Resource;

import java.util.Date;
import java.util.Map;
import java.util.UUID;

import org.apache.commons.lang3.tuple.ImmutablePair;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.core.MessageBuilder;
import org.springframework.amqp.core.MessageDeliveryMode;
import org.springframework.amqp.rabbit.core.RabbitTemplate;

@Component("q.message-sender")
public class MessageSender {
    public ImmutablePair<String, Boolean> isUp() {
        return rabbitTemplate.execute(channel -> {
            Map<String, Object> properties = channel.getConnection().getServerProperties();
            final var version = properties.get("version");
            return ImmutablePair.of(version == null ? null : version.toString(),
                    channel.isOpen());
        });

    }

    public <T extends Message> void produce(String queue, T message) {
        this.produce(queue, null, message);
    }

    public <T extends Message> void produce(String queue, String replyTo, T message) {
        this.send("", queue, UUID.randomUUID().toString(), PROTOBUF_CONTENT_TYPE, replyTo, message.getClass(),
                message.toByteArray());
    }

    private <T> void send(@Nullable String exchange, @Nullable String routingKey, @Nullable String messageId,
            @Nullable String contentType, String replyTo,
            Class<T> clazz,
            @Nullable byte[] body) {
        logger.info("send message({} bytes) ({}, {}) to ({},{})", body.length, contentType, messageId, exchange,
                routingKey);
        var builder = MessageBuilder.withBody(body).setMessageId(messageId).setContentType(contentType)
                .setTimestamp(new Date())
                .setType(clazz.getTypeName())
                .setDeliveryMode(MessageDeliveryMode.PERSISTENT);
        if (replyTo != null) {
            builder = builder.setReplyTo(replyTo);
        }
        final var message = builder.build();
        rabbitTemplate.send(exchange, routingKey, message);
    }

    // https://protobuf.dev/reference/protobuf/mime-types/
    public final static String PROTOBUF_CONTENT_TYPE = "application/protobuf";

    @Resource
    RabbitTemplate rabbitTemplate;
    private static final Logger logger = LoggerFactory.getLogger(MessageSender.class);
}
