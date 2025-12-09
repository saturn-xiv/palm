package com.github.saturn_xiv.palm.hyacinth.servlets;

import java.io.IOException;
import java.util.Map;
import java.util.stream.Collectors;

import org.apache.http.HttpStatus;
import org.eclipse.jetty.http.HttpMethod;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import jakarta.servlet.http.HttpServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import io.grpc.ManagedChannel;

import com.github.saturn_xiv.palm.hyacinth.HttpHeaders;
import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;
import com.github.saturn_xiv.palm.hyacinth.services.GRpcService;

public final class ApiServlet extends HttpServlet {
    public ApiServlet(Map<String, ManagedChannel> channels) {
        this.rpcService = new GRpcService(channels);
    }

    @Override
    protected void service(HttpServletRequest request, HttpServletResponse response) throws IOException {

        final var method = request.getMethod();
        final var path = request.getRequestURI();
        final var authorization = request.getHeader(HttpHeaders.AUTHORIZATION);
        final var x_real_ip = request.getHeader(HttpHeaders.X_REAL_IP);
        final var requestBody = request.getReader().lines().collect(Collectors.joining(System.lineSeparator()));
        final var requestType = request.getParameter("q");
        logger.info("{} {}", method, path);

        if (HttpMethod.POST.is(method)) {
            // /api/{host}/{package}/{service}/{method}?q=com.google.protobuf.Empty
            final var items = path.split("/");
            if (items == null || items.length != 6) {
                HttpHeaders.text(response, HttpStatus.SC_BAD_REQUEST);
                return;
            }

            final var req = new HttpRequest(x_real_ip, authorization, items[3], items[4], items[5], requestType,
                    requestBody);
            final var reply = rpcService.call(items[2], req);
            if (reply.left == HttpStatus.SC_OK) {
                HttpHeaders.json(response, reply.right);
                return;
            }
            logger.error("{} {}", reply.left, reply.right);
            HttpHeaders.text(response, reply.left, reply.right);
            return;
        }

        HttpHeaders.not_found(response);
    }

    private final GRpcService rpcService;
    private final static Logger logger = LoggerFactory.getLogger(ApiServlet.class);
}
