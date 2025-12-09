package com.github.saturn_xiv.palm.hyacinth.services;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.http.HttpStatus;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.grpc.ManagedChannel;
import io.grpc.Status;
import io.grpc.StatusRuntimeException;
import io.grpc.health.v1.HealthCheckRequest;
import io.grpc.health.v1.HealthGrpc;
import io.grpc.health.v1.HealthCheckResponse.ServingStatus;
import com.google.protobuf.Empty;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.util.JsonFormat;
import com.github.saturn_xiv.palm.hyacinth.handlers.ProtobufHandler;
import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;

public class GRpcService {
    public GRpcService(Map<String, ManagedChannel> channels) {
        this.channels = channels;
        this.protobufHandler = new ProtobufHandler(channels);
    }

    public Map<String, Optional<ServingStatus>> healthCheck() {
        var items = new HashMap<String, Optional<ServingStatus>>();
        for (var it : this.channels.entrySet()) {
            logger.debug("check health of {}", it.getKey());
            try {
                var stub = HealthGrpc.newBlockingStub(it.getValue());
                var request = HealthCheckRequest.getDefaultInstance();
                var response = stub.check(request);
                items.put(it.getKey(), Optional.of(response.getStatus()));
            } catch (StatusRuntimeException e) {
                logger.error("", e);
                items.put(it.getKey(), Optional.empty());
            }
        }
        return items;
    }

    public ImmutablePair<Integer, String> call(String host, HttpRequest request) {
        final var reply = this.protobufHandler.handle(host, request);
        final var status = reply.status();
        if (status.isOk()) {
            var it = reply.body().orElse(Empty.newBuilder());
            try {
                String body = JsonFormat.printer()
                        .alwaysPrintFieldsWithNoPresence()
                        .omittingInsignificantWhitespace()
                        .print(it);
                logger.info("{} {} bytes", reply.status().getCode(), body.length());
                return ImmutablePair.of(HttpStatus.SC_OK, body);
            } catch (InvalidProtocolBufferException e) {
                logger.error("", e);
                return ImmutablePair.of(HttpStatus.SC_INTERNAL_SERVER_ERROR, e.getMessage());
            }
        } else if (status == Status.NOT_FOUND
                || status == Status.UNIMPLEMENTED) {
            return ImmutablePair.of(HttpStatus.SC_NOT_FOUND, status.getDescription());
        } else if (status == Status.PERMISSION_DENIED
                || status == Status.UNAUTHENTICATED) {
            return ImmutablePair.of(HttpStatus.SC_FORBIDDEN, status.getDescription());
        }
        return ImmutablePair.of(HttpStatus.SC_INTERNAL_SERVER_ERROR, status.getDescription());
    }

    private ProtobufHandler protobufHandler;
    private Map<String, ManagedChannel> channels;

    private final static Logger logger = LoggerFactory.getLogger(GRpcService.class);
}
