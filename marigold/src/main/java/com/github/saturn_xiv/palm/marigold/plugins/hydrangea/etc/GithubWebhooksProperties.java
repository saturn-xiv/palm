package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.etc;

import java.io.Serializable;
import java.util.Map;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.validation.annotation.Validated;

import com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models.GitWebhook;

import jakarta.validation.Valid;

@Validated
@ConfigurationProperties(prefix = "palm.hydrangea.github")
public record GithubWebhooksProperties(Map<String, @Valid GitWebhook> webhooks) implements Serializable {

}
