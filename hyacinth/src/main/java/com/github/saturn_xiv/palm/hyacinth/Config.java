package com.github.saturn_xiv.palm.hyacinth;

import java.io.Serializable;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.fasterxml.jackson.annotation.JsonProperty;
import io.grpc.Grpc;
import io.grpc.InsecureChannelCredentials;
import io.grpc.ManagedChannel;

public final class Config implements Serializable {
    public Config() {
        this.nodes = new HashMap<>();
    }

    public Map<String, ManagedChannel> open() {
        Map<String, ManagedChannel> items = new HashMap<>();
        for (final var it : this.nodes.entrySet()) {
            logger.debug("open {} tcp://{}:{}", it.getKey(), it.getValue().host, it.getValue().port);
            var ch = Grpc
                    .newChannelBuilderForAddress(it.getValue().host, it.getValue().port,
                            InsecureChannelCredentials.create())
                    .build();
            items.put(it.getKey(), ch);
        }
        return items;
    }

    public final static class Node {
        public String host;
        public int port;
    }

    @JsonProperty("allowed-origins")
    public Set<String> allowedOrigins;
    public Map<String, Node> nodes;
    private static final Logger logger = LoggerFactory.getLogger(Config.class);
}
