package com.github.saturn_xiv.palm.hyacinth;

import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.server.ServerConnector;
import org.eclipse.jetty.util.thread.QueuedThreadPool;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class HttpServer {
    public HttpServer(Config config) {
        this.config = config;
    }

    // https://jetty.org/docs/jetty/12.1/programming-guide/server/http.html
    public void start(String name, int port) {
        var threadPool = new QueuedThreadPool();
        threadPool.setName(name);
        var server = new Server(threadPool);
        var connector = new ServerConnector(server);
        connector.setHost("127.0.0.1");
        connector.setPort(port);
        server.addConnector(connector);
        server.setHandler(new HttpHandler(this.config.nodes));

        logger.info("listening on http://127.0.0.1:{}", port);
        try {
            server.start();
        } catch (Exception e) {
            logger.error("failed to start server", e);
        }
    }

    private final Config config;
    private final static Logger logger = LoggerFactory.getLogger(HttpServer.class);
}
