package com.github.saturn_xiv.palm.camellia.services;

import java.io.IOException;

import jakarta.annotation.PostConstruct;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;

@Service("palm.camellia.otp-erlang-service")
public class OtpErlangService {
    @PostConstruct
    void init() throws IOException {
        logger.info("start opt-erlang server {}@{}", mailboxName, nodeName);
        OtpNode node = new OtpNode(nodeName);
        OtpMbox mbox = node.createMbox(mailboxName);
        OtpErlangObject o;
        OtpErlangTuple msg;
        OtpErlangPid from;

        while (true) {
            try {
                o = mbox.receive();
                if (o instanceof OtpErlangTuple) {
                    msg = (OtpErlangTuple) o;
                    from = (OtpErlangPid) (msg.elementAt(0));
                    logger.debug("receive message {}@{}", from.serial(), from.node());
                    mbox.send(from, msg.elementAt(1));
                }
            } catch (Exception e) {
                System.out.println("" + e);
            }
        }
    }

    @Value("${opt-erlang.node-name}")
    String nodeName;
    @Value("${opt-erlang.mail-box-name}")
    String mailboxName;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangService.class);
}
