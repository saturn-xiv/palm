package com.github.saturn_xiv.palm.hyacinth.filters;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

import jakarta.servlet.Filter;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.ServletRequest;
import jakarta.servlet.ServletResponse;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.google.common.base.Stopwatch;

public final class LoggingFilter implements Filter {

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain)
            throws IOException, ServletException {
        Stopwatch watch = Stopwatch.createStarted();
        chain.doFilter(request, response);
        logger.info("time elapsed {}", watch.elapsed(TimeUnit.MILLISECONDS));
    }

    private final static Logger logger = LoggerFactory.getLogger(LoggingFilter.class);
}
