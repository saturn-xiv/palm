package com.github.saturn_xiv.palm.camellia.requests;

import java.util.Optional;

import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;

public record FileUploadRequest(@NotNull @Size(min = 2, max = 63) String resourceType, Optional<Integer> resourceId) {

}
