package com.github.saturn_xiv.palm.hyacinth.handlers;

import java.net.URI;
import java.net.URL;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.Date;
import java.util.Map;
import java.util.Optional;

import org.apache.http.HttpStatus;
import org.apache.http.NameValuePair;
import org.apache.http.client.utils.URLEncodedUtils;
import org.apache.http.util.CharsetUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.grpc.ManagedChannel;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.handler.codec.http.DefaultHttpResponse;
import io.netty.handler.codec.http.FullHttpRequest;
import io.netty.handler.codec.http.HttpHeaderNames;
import io.netty.handler.codec.http.HttpHeaderValues;
import io.netty.handler.codec.http.HttpMethod;
import io.netty.handler.codec.http.HttpResponse;
import io.netty.handler.codec.http.HttpResponseStatus;
import io.netty.handler.codec.http.HttpUtil;
import io.netty.util.CharsetUtil;
import tools.jackson.databind.ObjectMapper;
import tools.jackson.databind.json.JsonMapper;

import com.github.saturn_xiv.palm.hyacinth.HttpHeaders;
import com.github.saturn_xiv.palm.hyacinth.models.HealthCheckResponse;
import com.github.saturn_xiv.palm.hyacinth.models.HttpRequest;
import com.github.saturn_xiv.palm.hyacinth.services.GRpcService;

public final class HttpServerHandler extends SimpleChannelInboundHandler<FullHttpRequest> {
    public HttpServerHandler(Map<String, ManagedChannel> channels, String version) {
        this.rpcService = new GRpcService(channels);
        this.version = version;
        this.objectMapper = JsonMapper.builder().build();
    }

    @Override
    protected void channelRead0(ChannelHandlerContext ctx, FullHttpRequest request) throws Exception {
        final var method = request.method();
        final var uri = request.uri();
        final var httpVersion = request.protocolVersion();
        final var x_real_ip = request.headers().get(HttpHeaders.X_REAL_IP);
        final var authorization = request.headers().get(HttpHeaders.AUTHORIZATION);
        final var path = new URI(uri).getPath();
        final var requestBody = request.content().toString(CharsetUtil.UTF_8);
        final var params = URLEncodedUtils.parse(new URI(uri).getQuery(), StandardCharsets.UTF_8);

        logger.info("{} {} {}", httpVersion, method, uri);
        if (HttpMethod.GET.equals(method) && "/health-check".equals(uri)) {
            final var rpc = this.rpcService.healthCheck();
            final var buf = this.objectMapper
                    .writeValueAsBytes(new HealthCheckResponse(Optional.empty(), rpc, version, new Date()));
            HttpHeaders.render(ctx, httpVersion, HttpResponseStatus.OK, HttpHeaders.APPLICATION_JSON_UTF8, buf);
            return;
        }
        // /api/{host}/{package}/{service}/{method}?q=com.google.protobuf.Empty
        if (HttpMethod.POST.equals(method) && path.startsWith("/api/") && params.size() == 1) {
            final var items = path.split("/");
            if (items.length == 6 && "q".equals(params.getFirst().getName())) {
                final var req = new HttpRequest(x_real_ip, authorization, items[3], items[4], items[5],
                        params.getFirst().getValue(),
                        requestBody);
                final var reply = rpcService.call(items[2], req);
                if (reply.left == HttpStatus.SC_OK) {
                    HttpHeaders.render(ctx, httpVersion, HttpResponseStatus.OK, HttpHeaders.APPLICATION_JSON_UTF8,
                            reply.right);
                } else {
                    logger.error("{} {}", reply.left, reply.right);
                    HttpHeaders.render(ctx, httpVersion, HttpResponseStatus.valueOf(reply.left), HttpHeaders.TEXT_PLAIN,
                            reply.right);
                }
                return;
            }
            HttpHeaders.render(ctx, httpVersion, HttpResponseStatus.BAD_REQUEST, HttpHeaders.TEXT_PLAIN_UTF8, "");
            return;
        }

        HttpHeaders.render(ctx, httpVersion, HttpResponseStatus.NOT_FOUND, HttpHeaders.TEXT_PLAIN_UTF8, "");
    }

    @Override
    public void channelReadComplete(ChannelHandlerContext ctx) throws Exception {
        ctx.flush();
        ctx.close();
    }

    private final GRpcService rpcService;
    private final String version;
    private final ObjectMapper objectMapper;
    private static final Logger logger = LoggerFactory.getLogger(HttpServerHandler.class);

}
