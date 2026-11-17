package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.controllers.api;

import java.io.IOException;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.etc.GogsWebhooksProperties;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.services.JobLogService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Succeed;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;

@RestController("portal.c.api.gogs")
@RequestMapping("/api/gogs")
public class GogsController {
    // https://gogs.io/advancing/webhooks
    @PostMapping(value = "/webhooks/{name}", produces = MediaType.APPLICATION_JSON_VALUE, consumes = MediaType.APPLICATION_JSON_VALUE)
    public Succeed webhooks(HttpServletRequest request, @PathVariable String name,
            @RequestHeader(value = "X-Gogs-Delivery", required = true) String delivery,
            @RequestHeader(value = "X-Gogs-Event", required = true) String event,
            @RequestHeader(value = "X-Gogs-Signature", required = true) String signature, @RequestBody String body)
            throws NoSuchAlgorithmException, InvalidKeyException, IOException {
        logger.info("receive Gogs webhook {} {}", event, delivery);

        Map<String, String> headers = new HashMap<>();
        headers.put("delivery", delivery);
        headers.put("event", event);
        final var summary = String.format("called by Gogs webhook %s %s", event, delivery);
        final var hook = webhooksProperties.webhooks().get(name);
        hook.handle(request, jobLogService, messageSender, headers, this.queue, name, summary, body);

        return new Succeed();
    }

    @PostConstruct
    void init() {
        this.webhooksProperties.webhooks().forEach((key, val) -> {
            logger.info("registered Gogs webhook {} v{}", key, val.version());
        });
    }

    @Resource
    GogsWebhooksProperties webhooksProperties;
    @Resource
    MessageSender messageSender;
    @Value("${palm.queues.hydrangea.shell-run}")
    String queue;
    @Resource
    JobLogService jobLogService;

    private final static Logger logger = LoggerFactory.getLogger(GogsController.class);
}
