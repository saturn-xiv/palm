package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses;

import java.io.Serializable;
import java.time.Instant;

public class Succeed implements Serializable {
    public Succeed() {
        this.createdAt = Instant.now();
    }

    private Instant createdAt;

    public Instant getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Instant createdAt) {
        this.createdAt = createdAt;
    }

}
