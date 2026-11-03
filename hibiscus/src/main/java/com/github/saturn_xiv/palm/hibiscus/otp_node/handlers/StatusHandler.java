package com.github.saturn_xiv.palm.hibiscus.otp_node.handlers;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.github.saturn_xiv.palm.hibiscus.otp_node.Handler;

@Component("otp-node.h.status")
public class StatusHandler extends Handler {

    @Override
    public String mailbox() {
        return "status";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action,
            int len) {
        if (action.equals("version") && len == 2) {
            super.send(mbox, fromPid, new OtpErlangObject[] {
                    super.ok(),
                    new OtpErlangString(version)
            });
            return;
        }

        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Value("${hibiscus.version}")
    String version;

    private static final Logger logger = LoggerFactory.getLogger(StatusHandler.class);
}
