package com.github.saturn_xiv.palm.hibiscus.otp_node.handlers;

import java.security.GeneralSecurityException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangBinary;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.github.saturn_xiv.palm.hibiscus.otp_node.Handler;
import com.github.saturn_xiv.palm.hibiscus.utils.Mac;

import jakarta.annotation.Resource;

@Component("otp-node.h.mac")
public class MacHandler extends Handler {
    @Override
    public String mailbox() {
        return "mac";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action, int len) {
        try {
            if (action.equals("compute") && len == 3 && request.elementAt(2) instanceof OtpErlangBinary message) {
                final var mac = this.mac.compute(message.binaryValue());
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok(),
                        new OtpErlangBinary(mac)
                });
                return;
            }
            if (action.equals("verify") && len == 4 && request.elementAt(2) instanceof OtpErlangBinary mac
                    && request.elementAt(3) instanceof OtpErlangBinary message) {
                this.mac.verify(mac.binaryValue(), message.binaryValue());
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok()
                });
                return;
            }

        } catch (GeneralSecurityException e) {
            logger.error("mac", e);
            super.internalServerError(mbox, fromPid);
            return;
        }
        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Resource
    Mac mac;
    private static final Logger logger = LoggerFactory.getLogger(MacHandler.class);

}
