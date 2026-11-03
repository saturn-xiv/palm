package com.github.saturn_xiv.palm.hibiscus.controllers.responses;

import java.io.Serializable;
import java.util.Date;

public class HealthCheckResponse implements Serializable {
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
