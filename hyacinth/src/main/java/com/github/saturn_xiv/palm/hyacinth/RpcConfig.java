package com.github.saturn_xiv.palm.hyacinth;

import java.util.HashMap;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.stereotype.Component;
import io.grpc.Grpc;
import io.grpc.InsecureChannelCredentials;
import io.grpc.ManagedChannel;

@Component
@EnableConfigurationProperties
@ConfigurationProperties(prefix = "hyacinth.rpc")
public class RpcConfig {

    public Map<String, ManagedChannel> open() {
        Map<String, ManagedChannel> items = new HashMap<>();
        for (var it : this.endpoints.entrySet()) {
            final var name = it.getKey();
            final var host = it.getValue().get("host");
            final var port = Integer.parseInt(it.getValue().get("port"));
            logger.debug("open {} tcp://{}:{}", name, host, port);
            var ch = Grpc
                    .newChannelBuilderForAddress(host, port, InsecureChannelCredentials.create())
                    .build();
            items.put(name, ch);
        }
        return items;
    }

    private Map<String, Map<String, String>> endpoints;
    private static final Logger logger = LoggerFactory.getLogger(RpcConfig.class);

    public Map<String, Map<String, String>> getEndpoints() {
        return endpoints;
    }

    public void setEndpoints(Map<String, Map<String, String>> endpoints) {
        this.endpoints = endpoints;
    }

}
