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
import com.github.saturn_xiv.palm.hibiscus.utils.Aead;

import jakarta.annotation.Resource;

@Component("otp-node.h.aead")
public class AeadHandler extends Handler {

    @Override
    public String mailbox() {
        return "aead";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action, int len) {
        try {
            if (action.equals("encrypt") && len == 4 && request.elementAt(2) instanceof OtpErlangBinary plain
                    && request.elementAt(3) instanceof OtpErlangBinary associated) {
                final var cipher = aead.encrypt(plain.binaryValue(), associated.binaryValue());
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok(),
                        new OtpErlangBinary(cipher)
                });
                return;
            }
            if (action.equals("decrypt") && len == 4 && request.elementAt(2) instanceof OtpErlangBinary cipher
                    && request.elementAt(3) instanceof OtpErlangBinary associated) {
                final var plain = aead.decrypt(cipher.binaryValue(), associated.binaryValue());
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok(),
                        new OtpErlangBinary(plain)
                });
                return;
            }
        } catch (GeneralSecurityException e) {
            logger.error("aead", e);
            super.internalServerError(mbox, fromPid);
            return;
        }
        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Resource
    Aead aead;
    private static final Logger logger = LoggerFactory.getLogger(AeadHandler.class);

}
