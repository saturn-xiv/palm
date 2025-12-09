package com.github.saturn_xiv.palm.hyacinth;

import java.util.EnumSet;
import java.util.Set;

import org.eclipse.jetty.ee10.servlet.ServletContextHandler;
import org.eclipse.jetty.http.HttpMethod;
import org.eclipse.jetty.server.Server;
import org.eclipse.jetty.server.ServerConnector;
import org.eclipse.jetty.server.handler.CrossOriginHandler;
import org.eclipse.jetty.util.thread.QueuedThreadPool;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.github.saturn_xiv.palm.hyacinth.filters.LoggingFilter;
import com.github.saturn_xiv.palm.hyacinth.servlets.ApiServlet;
import com.github.saturn_xiv.palm.hyacinth.servlets.HealthCheckServlet;

import jakarta.servlet.DispatcherType;

public class HttpServer {
    public HttpServer(Config config) {
        this.config = config;
    }

    // https://jetty.org/docs/jetty/12.1/programming-guide/server/http.html
    public void start(String name, int port, String version) {
        var channels = this.config.open();

        var threadPool = new QueuedThreadPool();
        threadPool.setName(name);
        var server = new Server(threadPool);
        var connector = new ServerConnector(server);
        connector.setHost("127.0.0.1");
        connector.setPort(port);
        server.addConnector(connector);

        CrossOriginHandler crossOriginHandler = new CrossOriginHandler();
        crossOriginHandler.setAllowedOriginPatterns(config.allowedOrigins);
        crossOriginHandler.setAllowCredentials(true);
        crossOriginHandler.setAllowedMethods(Set.of(HttpMethod.GET.toString(), HttpMethod.POST.toString()));
        server.setHandler(crossOriginHandler);

        ServletContextHandler context = new ServletContextHandler();
        context.setContextPath("/");
        crossOriginHandler.setHandler(context);

        context.addFilter(LoggingFilter.class, "/*", EnumSet.of(DispatcherType.REQUEST));
        {
            var servlet = new HealthCheckServlet(channels, version);
            context.addServlet(servlet, "/health-check");
        }
        {
            var servlet = new ApiServlet(channels);
            context.addServlet(servlet, "/api/*");
        }

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
