package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses;

import java.io.Serializable;
import java.util.Date;

public class HealthCheckResponse implements Serializable {
    private static final long serialVersionUID = 1L;

    public HealthCheckResponse() {
        this.createdAt = new Date();
    }

    private Date createdAt;

    public Date getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Date createdAt) {
        this.createdAt = createdAt;
    }

}
