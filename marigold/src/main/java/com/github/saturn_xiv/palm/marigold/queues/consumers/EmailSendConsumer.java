package com.github.saturn_xiv.palm.marigold.queues.consumers;

import java.io.IOException;
import java.io.UnsupportedEncodingException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.rabbit.annotation.Exchange;
import org.springframework.amqp.rabbit.annotation.Queue;
import org.springframework.amqp.rabbit.annotation.QueueBinding;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.amqp.support.AmqpHeaders;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.ByteArrayResource;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.mail.javamail.MimeMessageHelper;
import org.springframework.stereotype.Component;
import org.springframework.messaging.handler.annotation.Header;

import com.github.saturn_xiv.palm.plugins.email.v1.Task;
import com.github.saturn_xiv.palm.plugins.email.v1.Task.Address;
import com.rabbitmq.client.Channel;

import jakarta.annotation.Resource;
import jakarta.mail.MessagingException;
import jakarta.mail.internet.AddressException;
import jakarta.mail.internet.InternetAddress;

@Component("q.c.email-sender")
public class EmailSendConsumer {

    @RabbitListener(bindings = @QueueBinding(value = @Queue(value = "${palm.rabbitmq.email-send.queue}", durable = "true"), exchange = @Exchange(value = "${palm.rabbitmq.email-send.exchange}", type = "topic"), key = "${palm.rabbitmq.email-send.queue}"))
    public void handle(byte[] payload, Channel channel, @Header(AmqpHeaders.DELIVERY_TAG) long deliveryTag,
            @Header(AmqpHeaders.MESSAGE_ID) String messageId,
            @Header(AmqpHeaders.CONTENT_TYPE) String contentType) throws IOException, MessagingException {
        logger.info("receive message {}://{}", contentType, messageId);
        channel.basicAck(deliveryTag, false);

        final var task = Task.parseFrom(payload);
        var message = mailSender.createMimeMessage();
        var helper = new MimeMessageHelper(message, true, "UTF-8");
        helper.setSubject(task.getSubject());
        helper.setFrom(this.from);
        for (var it : task.getToList()) {
            helper.addTo(toAddress(it));
        }
        for (var it : task.getCcList()) {
            helper.addCc(toAddress(it));
        }
        for (var it : task.getBccList()) {
            helper.addBcc(toAddress(it));
        }
        {
            var body = task.getBody();
            helper.setText(body.getContent(), body.getHtml());
        }
        for (var it : task.getAttachmentsList()) {
            var buf = it.getContent();
            var res = new ByteArrayResource(buf.toByteArray()) {
                @Override
                public String getFilename() {
                    return it.getName();
                }
            };
            if (it.hasInlineId()) {
                helper.addInline(it.getInlineId(), res, it.getContentType());
            } else {
                helper.addAttachment(it.getName(), res, it.getContentType());
            }
        }

        mailSender.send(message);
    }

    private InternetAddress toAddress(Address it) throws UnsupportedEncodingException, AddressException {
        return it.hasName() ? new InternetAddress(it.getEmail(), it.getName()) : new InternetAddress(it.getEmail());
    }

    @Resource
    JavaMailSender mailSender;
    @Value("${spring.mail.username}")
    String from;

    private static final Logger logger = LoggerFactory.getLogger(EmailSendConsumer.class);
}
