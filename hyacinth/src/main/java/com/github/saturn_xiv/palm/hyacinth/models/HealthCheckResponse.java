package com.github.saturn_xiv.palm.hyacinth.models;

import java.util.Map;
import java.util.Optional;

public record HealthCheckResponse(Optional<String> database, Map<String, Optional<String>> rpcClients) {

}
