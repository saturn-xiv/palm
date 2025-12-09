package com.github.saturn_xiv.palm.hyacinth.servlets;

import java.io.IOException;
import java.util.Map;

import org.eclipse.jetty.http.HttpMethod;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.grpc.ManagedChannel;
import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import tools.jackson.databind.ObjectMapper;
import tools.jackson.databind.json.JsonMapper;

import com.github.saturn_xiv.palm.hyacinth.HttpHeaders;
import com.github.saturn_xiv.palm.hyacinth.services.GRpcService;

public class HealthCheckServlet extends HttpServlet {
    public HealthCheckServlet(Map<String, ManagedChannel> channels) {
        this.rpcService = new GRpcService(channels);
        this.objectMapper = JsonMapper.builder().build();
    }

    @Override
    protected void service(HttpServletRequest request, HttpServletResponse response) throws IOException {
        final var method = request.getMethod();
        final var path = request.getPathInfo();
        logger.info("{} {}", method, path);

        if (HttpMethod.GET.is(method)) {
            final var res = this.rpcService.healthCheck();
            final var buf = this.objectMapper.writeValueAsString(res);
            HttpHeaders.json(response, buf);
            return;
        }
        HttpHeaders.not_found(response);
    }

    private final GRpcService rpcService;
    private final ObjectMapper objectMapper;
    public final static String PATH = "/health-check";
    private final static Logger logger = LoggerFactory.getLogger(HealthCheckServlet.class);
}
