package com.github.saturn_xiv.palm.hibiscus.controllers;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

import com.github.saturn_xiv.palm.hibiscus.services.AttachmentService;
import com.github.saturn_xiv.palm.hibiscus.services.LogService;

import jakarta.annotation.Resource;

@Controller("c.health-check")
@RequestMapping("/api/health-check")
public class HealthCheckController {

    @Resource
    LogService logService;
    @Resource
    AttachmentService attachmentService;
}
