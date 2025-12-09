package com.github.saturn_xiv.palm.hyacinth;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import io.netty.bootstrap.ServerBootstrap;
import io.netty.channel.ChannelFuture;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelOption;
import io.netty.channel.ChannelPipeline;
import io.netty.channel.EventLoopGroup;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.handler.codec.http.HttpObjectAggregator;
import io.netty.handler.codec.http.HttpRequestDecoder;
import io.netty.handler.codec.http.HttpResponseEncoder;
import io.netty.handler.codec.http.HttpServerCodec;
import io.netty.handler.logging.LogLevel;
import io.netty.handler.logging.LoggingHandler;

import com.github.saturn_xiv.palm.hyacinth.handlers.HttpServerHandler;

public final class HttpServer {
    public HttpServer(Config config) {
        this.config = config;
    }

    public void start(String name, int port, String version) throws InterruptedException {
        var channels = this.config.open();

        EventLoopGroup boss = new NioEventLoopGroup();
        EventLoopGroup worker = new NioEventLoopGroup();
        try {
            ServerBootstrap bootstrap = new ServerBootstrap();
            bootstrap.group(boss, worker)
                    .channel(NioServerSocketChannel.class)
                    .handler(new LoggingHandler(LogLevel.WARN))
                    .childHandler(new ChannelInitializer<SocketChannel>() {
                        @Override
                        public void initChannel(SocketChannel ch) throws Exception {
                            ChannelPipeline p = ch.pipeline();
                            // p.addLast(new HttpRequestDecoder());
                            p.addLast(new HttpServerCodec());
                            p.addLast(new HttpObjectAggregator(1 << 20));// 1MB
                            p.addLast(new HttpServerHandler(channels, version));
                        }
                    })
                    .option(ChannelOption.SO_BACKLOG, 128)
                    .childOption(ChannelOption.SO_KEEPALIVE, true);
            final String host = "127.0.0.1";
            logger.info("listening on http://{}:{}", host, port);
            ChannelFuture future = bootstrap.bind(host, port).sync();
            future.channel().closeFuture().sync();
        } finally {
            logger.warn("shutdown event loops...");
            worker.shutdownGracefully();
            boss.shutdownGracefully();
        }
    }

    private final Config config;
    private final static Logger logger = LoggerFactory.getLogger(HttpServer.class);
}
