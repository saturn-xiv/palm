package com.github.saturn_xiv.palm.hyacinth;

import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.grpc.Channel;
import io.grpc.Status;
import reactor.netty.http.server.HttpServerRequest;
import reactor.netty.http.server.HttpServerResponse;

import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;
import com.github.saturn_xiv.palm.hyacinth.models.HttpResponse;

public class ProtobufHandler {
    public ProtobufHandler(Map<String, Channel> channels) {
        this.channels = channels;
    }

    public void handle(HttpServerRequest request, HttpServerResponse response) {
    }

    HttpResponse handle(final HttpRequest request) {
        logger.debug("handle {}://{}/{}/{}?q={}&p={}", request.host(), request.package_(), request.service(),
                request.method(), request.requestType(), request.responseType());

        var channel = this.channels.get(request.host());
        if (channel == null) {
            return new HttpResponse(Status.NOT_FOUND, null);
        }
        // Empty request = Empty.newBuilder().build();
        // GreeterGrpc.GreeterBlockingStub blockingStub=
        // GreeterGrpc.newBlockingStub(channel);
        // response = blockingStub.sayHello(request);
        return new HttpResponse(Status.NOT_FOUND, null);
    }

    private final Map<String, Channel> channels;
    private static final Logger logger = LoggerFactory.getLogger(ProtobufHandler.class);
}
