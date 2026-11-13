package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.api;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.info.BuildProperties;
import org.springframework.dao.DataAccessException;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.HealthCheckResponse;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.SettingRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.AttachmentService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.LogService;

import jakarta.annotation.Resource;

@RestController("portal.c.api.home")
@RequestMapping("/api")
public class HomeController {

    @GetMapping("/health-check")
    @PreAuthorize("permitAll()")
    public HealthCheckResponse healthCheck() {
        var reply = new HealthCheckResponse(buildProperties);
        try {
            settingRepository.getDatabaseVersion();
            reply.setDatabase(true);
        } catch (DataAccessException e) {
            logger.error("check database status", e);
            reply.setDatabase(false);
        }
        return reply;
    }

    @Resource
    LogService logService;
    @Resource
    AttachmentService attachmentService;
    @Resource
    BuildProperties buildProperties;
    @Resource
    SettingRepository settingRepository;
    private final static Logger logger = LoggerFactory.getLogger(HomeController.class);
}
