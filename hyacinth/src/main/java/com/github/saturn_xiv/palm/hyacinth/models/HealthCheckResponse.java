package com.github.saturn_xiv.palm.hyacinth.models;

import java.util.Date;
import java.util.Map;
import java.util.Optional;

import io.grpc.health.v1.HealthCheckResponse.ServingStatus;

public record HealthCheckResponse(Optional<String> database,
        Map<String, Optional<ServingStatus>> rpcEndpoints, String version, Date createdAt) {

}
