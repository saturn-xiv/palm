package com.github.saturn_xiv.palm.hyacinth;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelFutureListener;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.http.DefaultFullHttpResponse;
import io.netty.handler.codec.http.FullHttpResponse;
import io.netty.handler.codec.http.HttpHeaderNames;
import io.netty.handler.codec.http.HttpResponseStatus;
import io.netty.handler.codec.http.HttpVersion;
import io.netty.util.CharsetUtil;

public final class HttpHeaders {

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
    public final static String AUTHORIZATION = "Authorization";
    public final static String X_REAL_IP = "X-Real-IP";
    public final static String Bearer = "Bearer ";
    public final static String APPLICATION_JSON_UTF8 = "application/json; charset=UTF-8";
    public final static String TEXT_PLAIN_UTF8 = "text/plain; charset=UTF-8";

    public static void render(ChannelHandlerContext ctx, HttpVersion version, HttpResponseStatus status,
            String contentType, String body) {
        render(ctx, version, status, contentType, Unpooled.copiedBuffer(body, CharsetUtil.UTF_8));
    }

    public static void render(ChannelHandlerContext ctx, HttpVersion version, HttpResponseStatus status,
            String contentType, byte[] body) {
        render(ctx, version, status, contentType, Unpooled.copiedBuffer(body));
    }

    public static void render(ChannelHandlerContext ctx, HttpVersion version, HttpResponseStatus status,
            String contentType, ByteBuf body) {
        FullHttpResponse response = new DefaultFullHttpResponse(version, status, body);
        response.headers().set(HttpHeaderNames.CONTENT_TYPE, contentType);
        response.headers().setInt(HttpHeaderNames.CONTENT_LENGTH, response.content().readableBytes());
        ctx.write(response);
        ctx.writeAndFlush(Unpooled.EMPTY_BUFFER).addListener(ChannelFutureListener.CLOSE);
    }
}
