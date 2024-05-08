package com.github.saturn_xiv.palm.tutorial;

import com.github.saturn_xiv.palm.plugins.morus.v1.markdown.Markdown;
import org.apache.thrift.TException;
import org.apache.thrift.protocol.TProtocol;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class MorusClient {

    public MorusClient(String host, int port) throws TException {
        this.client = new ThriftClient(host, port);
    }

    public String markdownToHtml(String text, boolean sanitize) throws TException {
        return this.client.launch(MARKDOWN, new ThriftClient.Handler<>() {
            @Override
            public String execute(TProtocol protocol) throws TException {
                var client = new Markdown.Client(protocol);
                return client.to_html(text, sanitize);
            }
        });
    }

    private final ThriftClient client;
    private final String MARKDOWN = "markdown";

    private final static Logger logger = LoggerFactory.getLogger(MorusClient.class);
}
