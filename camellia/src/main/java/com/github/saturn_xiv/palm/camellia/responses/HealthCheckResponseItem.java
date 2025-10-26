package com.github.saturn_xiv.palm.camellia.responses;

import java.io.Serializable;
import java.util.Optional;

public record HealthCheckResponseItem(String name, Optional<String> status) implements Serializable {
}
