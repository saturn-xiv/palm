package com.github.saturn_xiv.palm.hyacinth;

import java.io.IOException;
import java.io.PrintWriter;

import jakarta.servlet.http.HttpServletResponse;

import org.apache.http.HttpStatus;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufUtil;
import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelFutureListener;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.http.DefaultFullHttpResponse;
import io.netty.handler.codec.http.DefaultHttpResponse;
import io.netty.handler.codec.http.FullHttpRequest;
import io.netty.handler.codec.http.FullHttpResponse;
import io.netty.handler.codec.http.HttpHeaderNames;
import io.netty.handler.codec.http.HttpHeaderValues;
import io.netty.handler.codec.http.HttpResponse;
import io.netty.handler.codec.http.HttpResponseStatus;
import io.netty.handler.codec.http.HttpUtil;
import io.netty.handler.codec.http.HttpVersion;
import io.netty.util.CharsetUtil;

public final class HttpHeaders {

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
    public final static String AUTHORIZATION = "Authorization";
    public final static String X_REAL_IP = "X-Real-IP";
    public final static String Bearer = "Bearer ";
    public final static String APPLICATION_JSON_UTF8 = "application/json; charset=UTF-8";
    public final static String TEXT_PLAIN_UTF8 = "text/plain; charset=UTF-8";

    public final static String APPLICATION_JSON = "application/json";
    public final static String TEXT_PLAIN = "text/plain";
    public final static String UTF8 = "UTF-8";

    public static void json(HttpServletResponse response, byte[] body) throws IOException {
        PrintWriter out = response.getWriter();
        response.setStatus(HttpStatus.SC_OK);
        response.setContentType(APPLICATION_JSON);
        response.setCharacterEncoding(UTF8);
        out.print(body);
        out.flush();
    }

    public static void json(HttpServletResponse response, String body) throws IOException {
        PrintWriter out = response.getWriter();
        response.setStatus(HttpStatus.SC_OK);
        response.setContentType(APPLICATION_JSON);
        response.setCharacterEncoding(UTF8);
        out.print(body);
        out.flush();
    }

    public static void not_found(HttpServletResponse response) throws IOException {
        text(response, HttpStatus.SC_NOT_FOUND);
    }

    public static void text(HttpServletResponse response, int status) throws IOException {
        text(response, status, "");
    }

    public static void text(HttpServletResponse response, int status, String body) throws IOException {
        PrintWriter out = response.getWriter();
        response.setContentType(TEXT_PLAIN);
        response.setCharacterEncoding(UTF8);
        response.setStatus(status);
        out.print(body);
        out.flush();
    }

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
