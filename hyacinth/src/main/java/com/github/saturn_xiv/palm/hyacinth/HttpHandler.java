package com.github.saturn_xiv.palm.hyacinth;

import java.util.Map;

import org.eclipse.jetty.server.Handler;
import org.eclipse.jetty.server.Request;
import org.eclipse.jetty.server.Response;
import org.eclipse.jetty.util.Callback;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class HttpHandler extends Handler.Abstract {
    public HttpHandler(Map<String, Config.Node> nodes) {
        this.nodes = nodes;
    }

    @Override
    public boolean handle(Request request, Response response, Callback callback) {
        callback.succeeded();
        return true;
    }

    private void healthCheck() {
        for (var entry : this.nodes.entrySet()) {
            var client = new GRpcClient(entry.getKey(), entry.getValue().host,
                    entry.getValue().port);
            client.open();
            // to health check
        }
    }

    private final Map<String, Config.Node> nodes;
    private final static Logger logger = LoggerFactory.getLogger(HttpHandler.class);
}
