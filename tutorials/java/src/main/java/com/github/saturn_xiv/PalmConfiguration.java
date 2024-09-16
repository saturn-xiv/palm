package com.github.saturn_xiv;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.github.saturn_xiv.palm.utils.GrpcClientFactory;
import io.dropwizard.core.Configuration;
import jakarta.validation.Valid;
import jakarta.validation.constraints.NotNull;

public class PalmConfiguration extends Configuration {
    @Valid
    @NotNull
    @JsonProperty("atropa")
    GrpcClientFactory atropa = new GrpcClientFactory();

    public GrpcClientFactory getAtropa() {
        return atropa;
    }

    public void setAtropa(GrpcClientFactory atropa) {
        this.atropa = atropa;
    }
}
