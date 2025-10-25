package com.github.saturn_xiv.palm.camellia.services;

import java.io.IOException;

import jakarta.annotation.PostConstruct;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;

@Component("palm.camellia.otp-erlang-service")
public class OtpErlangService {
    @PostConstruct
    void init() throws IOException {
        logger.info("start opt-erlang server {}@{}", mailboxName, nodeName);
        OtpNode node = new OtpNode(nodeName, cookie);
        OtpMbox box = node.createMbox(mailboxName);

        while (true) {
            try {
                handle(node, box);
            } catch (OtpException e) {
                logger.error("message handler", e);
            }
        }
    }

    void handle(OtpNode node, OtpMbox box) throws OtpException {
        OtpErlangObject object = box.receive();
        if (object instanceof OtpErlangTuple) {
            OtpErlangTuple request = (OtpErlangTuple) object;

            OtpErlangPid from = (OtpErlangPid) request.elementAt(0);
            String name = ((OtpErlangString) request.elementAt(1)).stringValue();
            logger.debug("receive message({}) from {}@{}", name, from.serial(), from.node());
            String greeting = String.format("Hello, %s!", name);
            OtpErlangString greeting_otp = new OtpErlangString(greeting);
            OtpErlangTuple response = new OtpErlangTuple(new OtpErlangObject[] {
                    box.self(), greeting_otp
            });
            box.send(from, response);
        }
    }

    @Value("${opt-erlang.node-name}")
    String nodeName;
    @Value("${opt-erlang.mail-box-name}")
    String mailboxName;
    @Value("${opt-erlang.cookie}")
    String cookie;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangService.class);
}
