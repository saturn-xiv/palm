package com.github.saturn_xiv.palm.marigold.plugins.portal.models.user;

import java.io.Serializable;

public interface Info extends Serializable {
    public boolean isAvailable();

    public String label();
}
