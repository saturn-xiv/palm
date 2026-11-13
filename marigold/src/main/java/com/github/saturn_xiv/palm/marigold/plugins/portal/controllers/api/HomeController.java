package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.api;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.HealthCheckResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.AttachmentService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.LogService;

import jakarta.annotation.Resource;

@RestController("portal.c.api.home")
@RequestMapping("/api")
public class HomeController {

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

}
