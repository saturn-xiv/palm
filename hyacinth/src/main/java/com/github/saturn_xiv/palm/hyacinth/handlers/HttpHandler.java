package com.github.saturn_xiv.palm.hyacinth.handlers;

import java.util.Map;

import org.eclipse.jetty.server.Handler;
import org.eclipse.jetty.server.Request;
import org.eclipse.jetty.server.Response;
import org.eclipse.jetty.util.Callback;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.grpc.ManagedChannel;

import com.github.saturn_xiv.palm.hyacinth.services.GRpcService;

public class HttpHandler extends Handler.Abstract {
    public HttpHandler(Map<String, ManagedChannel> channels) {
        this.rpcService = new GRpcService(channels);
    }

    @Override
    public boolean handle(Request request, Response response, Callback callback) {
        callback.succeeded();
        return true;
    }

    private final GRpcService rpcService;
    private final static Logger logger = LoggerFactory.getLogger(HttpHandler.class);
}
