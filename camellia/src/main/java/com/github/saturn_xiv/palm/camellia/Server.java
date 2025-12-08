package com.github.saturn_xiv.palm.camellia;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Server {
    public Server(Config config) {
        this.config = config;
    }

    public void start(int port) {
        for (var entry : this.config.nodes.entrySet()) {
            var client = new GRpcClient(entry.getKey(), entry.getValue().host, entry.getValue().port);
            client.open();
            // to health check
        }
        logger.info("listing on http://127.0.0.1:{}", port);
    }

    private final Config config;
    private final static Logger logger = LoggerFactory.getLogger(Server.class);
}
