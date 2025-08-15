package com.github.saturn_xiv.palm.hyacinth;

import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.fasterxml.jackson.dataformat.toml.TomlMapper;
import io.grpc.Channel;
import io.grpc.ManagedChannelBuilder;

import com.github.saturn_xiv.palm.hyacinth.models.Config;
import com.google.protobuf.Empty;

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
    }

    void handle(final String host, final String package_, final String service, final String method,
            final String requestType, final String responseType) {
        logger.debug("handle {}://{}/{}/{}?q={}&p={}", host, package_, service, method, requestType, responseType);
    }

    void handle(final String host) {
        var channel = this.channels.get(host);
        if (channel == null) {
            this.notFound();
            return;
        }
        Empty request = Empty.newBuilder().build();
        // GreeterGrpc.GreeterBlockingStub blockingStub=
        // GreeterGrpc.newBlockingStub(channel);
        // response = blockingStub.sayHello(request);

    }

    private void notFound() {
        // TODO
    }

    private Map<String, Channel> channels;

    private static final Logger logger = LoggerFactory.getLogger(Server.class);
}
