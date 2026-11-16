package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.controllers.api;

import java.io.IOException;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.etc.GogsWebhooksProperties;
import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.services.JobLogService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.queues.MessageSender;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;

@RestController("portal.c.api.github")
@RequestMapping("/api/github")
public class GithubController {
    // https://docs.github.com/en/webhooks/using-webhooks/creating-webhooks#creating-a-repository-webhook
    // https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries#validating-webhook-deliveries
    // https://docs.github.com/en/webhooks/webhook-events-and-payloads#delivery-headers
    @GetMapping("/webhooks/{name}")
    public void webhooks(HttpServletRequest request, @PathVariable String name,
            @RequestHeader(value = "User-Agent", required = true) String userAgent,
            @RequestHeader(value = "X-GitHub-Hook-ID", required = true) String hookId,
            @RequestHeader(value = "X-GitHub-Delivery", required = true) String delivery,
            @RequestHeader(value = "X-GitHub-Event", required = true) String event,
            @RequestHeader(value = "X-Hub-Signature-256", required = true) String signature,
            @RequestHeader(value = "X-GitHub-Hook-Installation-Target-Type", required = true) String installationTargetType,
            @RequestHeader(value = "X-GitHub-Hook-Installation-Target-ID", required = true) String installationTargetId,
            @RequestBody String body)
            throws NoSuchAlgorithmException, InvalidKeyException, IOException {
        logger.info("receive Github webhook {} {} {}", hookId, event, delivery);
        final var summary = String.format("called by Github webhook %s %s", event, delivery);
        Map<String, String> headers = new HashMap<>();
        headers.put("userAgent", userAgent);
        headers.put("hookId", hookId);
        headers.put("delivery", delivery);
        headers.put("event", event);
        headers.put("installationTargetId", installationTargetId);
        headers.put("installationTargetType", installationTargetType);

        final var hook = webhooksProperties.webhooks().get(name);
        hook.handle(request, jobLogService, messageSender, headers, installationTargetId, name, summary, body);
    }

    @PostConstruct
    void init() {
        this.webhooksProperties.webhooks().forEach((key, val) -> {
            logger.info("registered Github webhook {} v{}", key, val.version());
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

    private final static Logger logger = LoggerFactory.getLogger(GithubController.class);
}
