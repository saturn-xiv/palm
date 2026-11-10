package com.github.saturn_xiv.palm.marigold.queues.consumers;

import java.io.IOException;
import java.util.Optional;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.rabbit.annotation.Queue;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;
import org.springframework.messaging.handler.annotation.Header;
import org.springframework.messaging.handler.annotation.Payload;
import org.springframework.amqp.support.AmqpHeaders;

import com.github.saturn_xiv.palm.marigold.utils.TwilioClient;
import com.github.saturn_xiv.palm.plugins.sms.v1.Task;
import com.rabbitmq.client.Channel;

import jakarta.annotation.Resource;

@Component("q.c.sms-send")
public class SmsSendConsumer {

    // @RabbitListener(bindings = @QueueBinding(value = @Queue(value =
    // "${palm.rabbitmq.sms-send.queue}", durable = "true"), exchange =
    // @Exchange(value = "${palm.rabbitmq.sms-send.exchange}", type = "topic"), key
    // = "${palm.rabbitmq.sms-send.queue}"))
    @RabbitListener(queuesToDeclare = @Queue(value = "${palm.queues.sms-send}", durable = "true"))
    public void handle(@Payload byte[] payload, Channel channel, @Header(AmqpHeaders.DELIVERY_TAG) long deliveryTag,
            @Header(AmqpHeaders.MESSAGE_ID) String messageId,
            @Header(AmqpHeaders.CONTENT_TYPE) String contentType) throws IOException {
        logger.info("receive message {}://{}", contentType, messageId);
        channel.basicAck(deliveryTag, false);
        final var task = Task.parseFrom(payload);
        switch (task.getBy()) {
            case TWILIO:
                for (var to : task.getToList()) {
                    twilioClient.sendSms(task.getFrom(), to, task.getBody(),
                            task.hasStatusCallback() ? Optional.of(task.getStatusCallback()) : Optional.empty());
                }
                break;
            default:
                logger.error("Unimplemented sms provider");
        }
    }

    @Resource
    TwilioClient twilioClient;

    private static final Logger logger = LoggerFactory.getLogger(SmsSendConsumer.class);
}
