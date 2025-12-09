package com.github.saturn_xiv.palm.hyacinth;

import java.io.IOException;
import java.io.PrintWriter;

import jakarta.servlet.http.HttpServletResponse;

import org.apache.http.HttpStatus;

public final class HttpHeaders {

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
    public final static String AUTHORIZATION = "Authorization";
    public final static String X_REAL_IP = "X-Real-IP";
    public final static String Bearer = "Bearer ";
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
}
