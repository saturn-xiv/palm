package com.github.saturn_xiv.palm.marigold.queues.consumers;

import java.io.IOException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.rabbit.annotation.Exchange;
import org.springframework.amqp.rabbit.annotation.Queue;
import org.springframework.amqp.rabbit.annotation.QueueBinding;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.amqp.support.AmqpHeaders;
import org.springframework.stereotype.Component;
import org.springframework.messaging.handler.annotation.Header;

import com.github.saturn_xiv.palm.plugins.email.v1.Task;
import com.rabbitmq.client.Channel;

@Component("q.c.email-sender")
public class EmailSendConsumer {

    @RabbitListener(bindings = @QueueBinding(value = @Queue(value = "${palm.rabbitmq.email-send.queue}", durable = "true"), exchange = @Exchange(value = "${palm.rabbitmq.email-send.exchange}", type = "topic"), key = "${palm.rabbitmq.email-send.queue}"))
    public void handle(byte[] payload, Channel channel, @Header(AmqpHeaders.DELIVERY_TAG) long deliveryTag,
            @Header(AmqpHeaders.MESSAGE_ID) String messageId,
            @Header(AmqpHeaders.CONTENT_TYPE) String contentType) throws IOException {
        logger.info("receive message {}://{}", contentType, messageId);
        channel.basicAck(deliveryTag, false);

        final var task = Task.parseFrom(payload);
    }

    private static final Logger logger = LoggerFactory.getLogger(EmailSendConsumer.class);
}
