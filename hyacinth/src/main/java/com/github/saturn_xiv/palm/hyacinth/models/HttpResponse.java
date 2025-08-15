package com.github.saturn_xiv.palm.hyacinth.models;

import java.util.Optional;

import com.google.protobuf.MessageOrBuilder;

import io.grpc.Status;

public record HttpResponse(Status status, Optional<MessageOrBuilder> body) {

}
