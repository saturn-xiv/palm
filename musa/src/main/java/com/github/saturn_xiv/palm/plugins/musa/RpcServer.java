package com.github.saturn_xiv.palm.plugins.musa;

import com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay.*;
import jakarta.annotation.PostConstruct;
import org.apache.thrift.TException;
import org.apache.thrift.TMultiplexedProcessor;
import org.apache.thrift.TProcessor;
import org.apache.thrift.protocol.TBinaryProtocol;
import org.apache.thrift.protocol.TProtocolFactory;
import org.apache.thrift.server.TServer;
import org.apache.thrift.server.TThreadPoolServer;
import org.apache.thrift.transport.TSSLTransportFactory;
import org.apache.thrift.transport.TServerSocket;
import org.apache.thrift.transport.TServerTransport;
import org.apache.thrift.transport.TTransportFactory;
import org.apache.thrift.transport.layered.TFramedTransport;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

@Component("palm.musa.server.rpc")
public class RpcServer {

    @PostConstruct
    void startUp() throws TException {
        TProtocolFactory protocolFactory = new TBinaryProtocol.Factory();
        TTransportFactory transportFactory = new TFramedTransport.Factory();
        TMultiplexedProcessor processor = new TMultiplexedProcessor();
        {
            final String name = Jsapi.class.getName();
            logger.debug("register wechatpay-jsapi service {}", name);
            processor.registerProcessor(name, new Jsapi.Processor<>(wechatPayJsapiService));
        }
        {
            final String name = Native.class.getName();
            logger.debug("register wechatpay-native service {}", name);
            processor.registerProcessor(name, new Native.Processor<>(wechatPayNativeService));
        }
        {
            final String name = Bill.class.getName();
            logger.debug("register wechatpay-bill service {}", name);
            processor.registerProcessor(name, new Bill.Processor<>(wechatPayBillService));
        }
        {
            final String name = Refund.class.getName();
            logger.debug("register wechatpay-refund service {}", name);
            processor.registerProcessor(name, new Refund.Processor<>(wechatPayRefundService));
        }
        {
            final String name = Transfer.class.getName();
            logger.debug("register wechatpay-transfer service {}", name);
            processor.registerProcessor(name, new Transfer.Processor<>(wechatPayTransferService));
        }

        if (this.secure) {
            this.start_https(protocolFactory, transportFactory, processor);
        } else {
            this.start_http(protocolFactory, transportFactory, processor);
        }
    }


    void start_http(TProtocolFactory protocolFactory, TTransportFactory transportFactory, TProcessor processor) throws TException {
        TServerTransport serverTransport = new TServerSocket(port);
        TServer server = new TThreadPoolServer(new TThreadPoolServer.Args(serverTransport).transportFactory(transportFactory).protocolFactory(protocolFactory).processor(processor));
        logger.info("start rpc server on tcp://0.0.0.0:{}", port);
        server.serve();
    }

    //    -Djavax.net.ssl.keyStore=.keystore and -Djavax.net.ssl.keyStorePassword=thrift
    void start_https(TProtocolFactory protocolFactory, TTransportFactory transportFactory, TProcessor processor) throws TException {
        TSSLTransportFactory.TSSLTransportParameters params = new TSSLTransportFactory.TSSLTransportParameters();
//        TODO load key files
        params.setKeyStore("", "", null, null);
        TServerTransport serverTransport = TSSLTransportFactory.getServerSocket(port, 0, null, params);
        TServer server = new TThreadPoolServer(new TThreadPoolServer.Args(serverTransport).transportFactory(transportFactory).protocolFactory(protocolFactory).processor(processor));
        logger.info("start rpc server on tcps://0.0.0.0:{}", port);
        server.serve();

    }

    @Value("${app.rpc.port}")
    int port;
    @Value("${app.rpc.secure}")
    boolean secure;
    @Value("${app.rpc.key-file}")
    String keyFile;
    @Value("${app.rpc.cert-file}")
    String certFile;
    @Value("${app.rpc.ca-file}")
    String caFile;


    @Autowired
    Jsapi.Iface wechatPayJsapiService;
    @Autowired
    Bill.Iface wechatPayBillService;
    @Autowired
    Native.Iface wechatPayNativeService;
    @Autowired
    Refund.Iface wechatPayRefundService;
    @Autowired
    Transfer.Iface wechatPayTransferService;


    private final static Logger logger = LoggerFactory.getLogger(RpcServer.class);

}
