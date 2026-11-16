package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models;

import java.util.Map;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.validation.annotation.Validated;

import jakarta.validation.Valid;

@Validated
@ConfigurationProperties(prefix = "palm.hydrangea.gogs")
public record GogsWebhooksProperties(@Valid Map<String, GogsWebhook> webhooks) {
}
