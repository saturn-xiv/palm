package com.github.saturn_xiv.palm.hyacinth.controllers;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.hyacinth.models.HealthCheckResponse;
import com.github.saturn_xiv.palm.hyacinth.services.LogService;

@RestController("palm.hyacinth.health-check-controller")
public class HealthCheckController {
    @GetMapping("/health-check")
    HealthCheckResponse show() {
        Map<String, Optional<String>> rpcClients = new HashMap<>();
        logger.debug("testing rpc client for {}", "jasmine");
        final var version = logService.version();
        return new HealthCheckResponse(Optional.of(version), rpcClients);
    }

    @Resource
    LogService logService;

    private static final Logger logger = LoggerFactory.getLogger(HealthCheckController.class);
}
