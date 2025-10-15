package com.github.saturn_xiv.palm.hyacinth.services;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

import org.apache.commons.lang3.tuple.ImmutablePair;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.stereotype.Component;
import io.grpc.Status;
import com.google.protobuf.Empty;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.util.JsonFormat;

import com.github.saturn_xiv.palm.hyacinth.ProtobufHandler;
import com.github.saturn_xiv.palm.hyacinth.RpcConfig;
import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;

@Component("palm.hyacinth.rpc-service")
public class RpcService {
    public ImmutablePair<HttpStatus, String> call(String host, HttpRequest request) {
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
                return ImmutablePair.of(HttpStatus.OK, body);
            } catch (InvalidProtocolBufferException e) {
                logger.error("", e);
                return ImmutablePair.of(HttpStatus.INTERNAL_SERVER_ERROR, e.getMessage());
            }
        } else if (status == Status.NOT_FOUND
                || status == Status.UNIMPLEMENTED) {
            return ImmutablePair.of(HttpStatus.NOT_FOUND, status.getDescription());
        } else if (status == Status.PERMISSION_DENIED
                || status == Status.UNAUTHENTICATED) {
            return ImmutablePair.of(HttpStatus.FORBIDDEN, status.getDescription());
        }
        return ImmutablePair.of(HttpStatus.INTERNAL_SERVER_ERROR, status.getDescription());
    }

    @PostConstruct
    public void init() {
        final var channels = this.rpcConfig.open();
        this.protobufHandler = new ProtobufHandler(channels);
    }

    @Resource
    RpcConfig rpcConfig;
    private ProtobufHandler protobufHandler;

    private static final Logger logger = LoggerFactory.getLogger(RpcService.class);
}
