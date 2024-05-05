package com.github.saturn_xiv.palm.plugins.musa;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.Jsapi;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import java.io.IOException;

@Component("palm.musa.server.rpc")
public class RpcServer {


    @PostConstruct
    void startUp() throws IOException {

        logger.info("Start gRPC server on tcp://0.0.0.0:{}", port);

    }


    @PreDestroy
    void shutDown() throws InterruptedException {
        logger.warn("Showdown gRPC server");

    }

    @Value("${app.rpc.port}")
    int port;
    @Value("${app.rpc.key-file}")
    String keyFile;
    @Value("${app.rpc.cert-file}")
    String certFile;
    @Value("${app.rpc.ca-file}")
    String caFile;


    @Autowired
    Jsapi.Iface wechatPayJsapiService;


    private final static Logger logger = LoggerFactory.getLogger(RpcServer.class);

}
