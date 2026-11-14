package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql;

import java.time.OffsetDateTime;
import java.time.ZoneOffset;

import org.springframework.boot.info.BuildProperties;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.stereotype.Controller;

import jakarta.annotation.Resource;

@Controller("portal.g.home")
public class HomeController {
    @QueryMapping
    public String apiVersion() {
        return buildProperties.getVersion();
    }

    @QueryMapping
    public OffsetDateTime buildTime() {
        return buildProperties.getTime().atOffset(ZoneOffset.UTC);
    }

    @Resource
    BuildProperties buildProperties;
}
