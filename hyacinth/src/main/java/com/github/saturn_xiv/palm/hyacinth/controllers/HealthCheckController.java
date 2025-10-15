package com.github.saturn_xiv.palm.hyacinth.controllers;

import java.util.Optional;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.hyacinth.models.HealthCheckResponse;
import com.github.saturn_xiv.palm.hyacinth.services.LogService;
import com.github.saturn_xiv.palm.hyacinth.services.RpcService;

@RestController("palm.hyacinth.health-check-controller")
public class HealthCheckController {
    @GetMapping("/health-check")
    HealthCheckResponse show() {
        final var rpc = rpcService.healthCheck();

        logger.debug("testing database {}", logService.driver());
        final var db = logService.version();
        return new HealthCheckResponse(Optional.of(db), rpc);
    }

    @Resource
    LogService logService;
    @Resource
    RpcService rpcService;

    private static final Logger logger = LoggerFactory.getLogger(HealthCheckController.class);
}
