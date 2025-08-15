package com.github.saturn_xiv.palm.hyacinth;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.fasterxml.jackson.dataformat.toml.TomlMapper;
import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.Status;
import io.netty.handler.codec.http.HttpHeaderNames;
import io.netty.handler.codec.http.HttpHeaderValues;
import io.netty.handler.codec.http.HttpResponseStatus;
import io.netty.handler.codec.http.QueryStringDecoder;
import reactor.core.publisher.Mono;
import reactor.netty.http.server.HttpServer;
import com.google.protobuf.Empty;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.util.JsonFormat;

import com.github.saturn_xiv.palm.hyacinth.models.Config;
import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;

public class Server {
    public Server(String config_file) throws IOException {
        this.channels = new HashMap<>();
        TomlMapper mapper = new TomlMapper();
        logger.info("load configuration from {}", config_file);
        Config config = mapper.readValue(new File(config_file), Config.class);
        for (var entry : config.backends().entrySet()) {
            var name = entry.getKey();
            var host = entry.getValue().host();
            var port = entry.getValue().port();
            logger.debug("found rpc backend {}(tcp://{}:{})", name, host, port);
            var channel = ManagedChannelBuilder.forAddress(host, port)
                    // .useTransportSecurity()
                    .usePlaintext().build();
            this.channels.put(name, channel);
        }

    }

    public void launch(int port) {
        logger.info("listen on http://0.0.0.0:{}", port);
        var handler = new ProtobufHandler(channels);
        var server = HttpServer.create()
                .port(port)
                .route(routes -> routes.post("/{host}/{package}/{service}/{method}",
                        (request, response) -> {
                            String uri = request.uri();
                            QueryStringDecoder decoder = new QueryStringDecoder(uri);
                            Map<String, List<String>> queryParams = decoder.parameters();
                            List<String> requestTypes = queryParams.get("q");
                            List<String> responseTypes = queryParams.get("p");
                            if (requestTypes == null || requestTypes.size() != 1 || responseTypes == null
                                    || responseTypes.size() != 1) {
                                return response.status(HttpResponseStatus.BAD_REQUEST)
                                        .header(HttpHeaderNames.CONTENT_TYPE, HttpHeaderValues.TEXT_PLAIN)
                                        .sendString(Mono.just(""));
                            }
                            return request.receive()
                                    .asString()
                                    .flatMap(requestBody -> {
                                        final var req = new HttpRequest(request.param("host"), request.param("package"),
                                                request.param("service"), request.param("method"),
                                                request.requestHeaders().get(HttpHeaderNames.AUTHORIZATION),
                                                requestTypes.getFirst(), requestBody,
                                                responseTypes.getFirst());
                                        var reply = handler.handle(req);
                                        if (reply.status().isOk()) {
                                            var it = reply.body().orElse(Empty.newBuilder());
                                            try {
                                                String body = JsonFormat.printer().print(it);
                                                return response.status(HttpResponseStatus.OK)
                                                        .header(HttpHeaderNames.CONTENT_TYPE,
                                                                HttpHeaderValues.APPLICATION_JSON)
                                                        .sendString(Mono.just(body));
                                            } catch (InvalidProtocolBufferException e) {
                                                logger.error("", e);
                                            }
                                        } else if (reply.status() == Status.NOT_FOUND
                                                || reply.status() == Status.UNIMPLEMENTED) {
                                            return response.status(HttpResponseStatus.NOT_FOUND)
                                                    .header(HttpHeaderNames.CONTENT_TYPE, HttpHeaderValues.TEXT_PLAIN)
                                                    .sendString(Mono.just(""));
                                        } else if (reply.status() == Status.PERMISSION_DENIED
                                                || reply.status() == Status.UNAUTHENTICATED) {
                                            return response.status(HttpResponseStatus.FORBIDDEN)
                                                    .header(HttpHeaderNames.CONTENT_TYPE, HttpHeaderValues.TEXT_PLAIN)
                                                    .sendString(Mono.just(""));
                                        }
                                        return response.status(HttpResponseStatus.INTERNAL_SERVER_ERROR)
                                                .header(HttpHeaderNames.CONTENT_TYPE, HttpHeaderValues.TEXT_PLAIN)
                                                .sendString(Mono.just(""));
                                    });

                        }));
        server.bindNow().onDispose().block();
    }

    private Map<String, Channel> channels;
    private static final Logger logger = LoggerFactory.getLogger(Server.class);
}
