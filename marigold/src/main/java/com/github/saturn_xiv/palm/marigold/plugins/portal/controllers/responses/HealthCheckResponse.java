package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses;

import java.io.Serializable;
import java.util.Date;

import org.springframework.boot.info.BuildProperties;

public class HealthCheckResponse implements Serializable {
    private static final long serialVersionUID = 1L;

    public HealthCheckResponse(BuildProperties buildProperties) {
        this.version = buildProperties.getVersion();
        this.buildTime = Date.from(buildProperties.getTime());
        this.createdAt = new Date();
    }

    private boolean database;
    private Date createdAt;
    private Date buildTime;
    private String version;

    public boolean isDatabase() {
        return database;
    }

    public void setDatabase(boolean database) {
        this.database = database;
    }

    public Date getBuildTime() {
        return buildTime;
    }

    public void setBuildTime(Date buildTime) {
        this.buildTime = buildTime;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(String version) {
        this.version = version;
    }

    public Date getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Date createdAt) {
        this.createdAt = createdAt;
    }

}
