package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.api.responses;

import java.io.Serializable;
import java.time.Instant;

import org.springframework.boot.info.BuildProperties;

public class HealthCheckResponse implements Serializable {
    private static final long serialVersionUID = 1L;

    public HealthCheckResponse(BuildProperties buildProperties) {
        this.version = buildProperties.getVersion();
        this.buildTime = buildProperties.getTime();
        this.createdAt = Instant.now();
    }

    private boolean database;
    private boolean cache;
    private boolean messageBroker;

    private Instant createdAt;
    private Instant buildTime;
    private String version;

    public boolean isCache() {
        return cache;
    }

    public void setCache(boolean cache) {
        this.cache = cache;
    }

    public boolean isMessageBroker() {
        return messageBroker;
    }

    public void setMessageBroker(boolean messageBroker) {
        this.messageBroker = messageBroker;
    }

    public boolean isDatabase() {
        return database;
    }

    public void setDatabase(boolean database) {
        this.database = database;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(String version) {
        this.version = version;
    }

    public Instant getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Instant createdAt) {
        this.createdAt = createdAt;
    }

    public Instant getBuildTime() {
        return buildTime;
    }

    public void setBuildTime(Instant buildTime) {
        this.buildTime = buildTime;
    }

}
