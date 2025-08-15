package com.github.saturn_xiv.palm.hyacinth.models;

public record HttpRequest(String host, String package_, String service, String method,
        String authorization, String requestType, String requestBody, String responseType) {
}
