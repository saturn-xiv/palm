package com.github.saturn_xiv.palm.hyacinth;

import java.lang.reflect.InvocationTargetException;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.grpc.ManagedChannel;
import io.grpc.Status;
import com.google.protobuf.InvalidProtocolBufferException;

import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;
import com.github.saturn_xiv.palm.hyacinth.models.HttpResponse;

public class ProtobufHandler {
    public ProtobufHandler(Map<String, ManagedChannel> channels) {
        this.channels = channels;
    }

    HttpResponse handle(final String host, final HttpRequest request) {
        logger.info("handle {}://{}/{}/{}?q={}", host, request.package_(), request.service(),
                request.method(), request.requestType());

        var channel = this.channels.get(host);
        if (channel == null) {
            return new HttpResponse(Status.NOT_FOUND, null);
        }
        try {
            var response = request.execute(channel);
            return new HttpResponse(Status.OK, response);
        } catch (ClassNotFoundException | NoSuchMethodException | IllegalAccessException
                | InvocationTargetException | InvalidProtocolBufferException e) {
            logger.error("", e);
            return new HttpResponse(Status.INVALID_ARGUMENT, null);
        }
    }

    private final Map<String, ManagedChannel> channels;
    private static final Logger logger = LoggerFactory.getLogger(ProtobufHandler.class);
}
