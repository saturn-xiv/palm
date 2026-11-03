package com.github.saturn_xiv.palm.hibiscus.controllers;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.hibiscus.controllers.responses.HealthCheckResponse;
import com.github.saturn_xiv.palm.hibiscus.services.AttachmentService;
import com.github.saturn_xiv.palm.hibiscus.services.LogService;
import com.github.saturn_xiv.palm.hibiscus.utils.CasbinEnforcer;

import jakarta.annotation.Resource;

@RestController("c.status")
@RequestMapping("/api")
public class StatusController {

    @GetMapping("/health-check")
    @PreAuthorize("permitAll()")
    public HealthCheckResponse healthCheck() {
        // TODO
        return new HealthCheckResponse();
    }

    @Resource
    LogService logService;
    @Resource
    AttachmentService attachmentService;
    @Resource
    CasbinEnforcer casbinEnforcer;
}
