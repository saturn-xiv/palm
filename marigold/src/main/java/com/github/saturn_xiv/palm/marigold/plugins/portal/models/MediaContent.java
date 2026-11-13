package com.github.saturn_xiv.palm.marigold.plugins.portal.models;

import java.io.Serializable;

public record MediaContent(String body) implements Serializable {
    private static final long serialVersionUID = 1L;

    public enum Editor {
        Textarea
    }
}
