package com.github.saturn_xiv.palm.marigold.plugins.portal.etc;

import java.util.Map;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.validation.annotation.Validated;

import com.github.saturn_xiv.palm.marigold.google.GoogleOauth2;

import jakarta.validation.Valid;

@Validated
@ConfigurationProperties(prefix = "palm.portal.google")
public record GoogleProperties(Map<String, @Valid GoogleOauth2> oauth2) {

}
