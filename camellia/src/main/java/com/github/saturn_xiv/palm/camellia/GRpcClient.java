package com.github.saturn_xiv.palm.camellia;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class GRpcClient {
    public GRpcClient(String name, String host, int port) {
        this.name = name;
        this.host = host;
        this.port = port;
    }

    public void open() {
        logger.debug("open gRPC client http://%s:%s for %s", this.host, this.port, this.name);
        // TODO
    }

    final String name;
    final String host;
    final int port;

    private final static Logger logger = LoggerFactory.getLogger(GRpcClient.class);
}
