package com.github.saturn_xiv.palm.camellia.services;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.ericsson.otp.erlang.OtpNode;

public class OtpErlangRunnableServer implements Runnable {
    public OtpErlangRunnableServer(OtpErlangServer instance, OtpNode node, String mailboxName) {
        this.instance = instance;
        this.node = node;
        this.mailboxName = mailboxName;
    }

    @Override
    public void run() {
        instance.launch(node, mailboxName);
    }

    private OtpErlangServer instance;
    private OtpNode node;
    final private String mailboxName;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangRunnableServer.class);
}
