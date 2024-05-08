package com.github.saturn_xiv.palm.tutorial;

import org.apache.thrift.TException;
import org.apache.thrift.protocol.TBinaryProtocol;
import org.apache.thrift.protocol.TMultiplexedProtocol;
import org.apache.thrift.protocol.TProtocol;
import org.apache.thrift.transport.TSocket;
import org.apache.thrift.transport.TTransport;
import org.apache.thrift.transport.layered.TFramedTransport;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.Serializable;

public class ThriftClient implements Serializable {
    public interface Handler<T> {
        T execute(TProtocol protocol) throws TException;
    }

    public ThriftClient(String host, int port) {
        this.host = host;
        this.port = port;
    }

    public <T> T launch(String service, Handler<T> handler) throws TException {
        logger.debug("open tcp://{}:{}", this.host, this.port);
        try (TSocket socket = new TSocket(this.host, this.port)) {
            socket.open();

            TTransport transport = new TFramedTransport(socket);
            TProtocol protocol = new TMultiplexedProtocol(new TBinaryProtocol(transport), service);

            return handler.execute(protocol);
        }
    }

    private final String host;
    private final int port;
    private final static Logger logger = LoggerFactory.getLogger(ThriftClient.class);
}
