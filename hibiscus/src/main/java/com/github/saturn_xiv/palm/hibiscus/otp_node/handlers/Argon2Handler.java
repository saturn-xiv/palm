package com.github.saturn_xiv.palm.hibiscus.otp_node.handlers;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangBinary;
import com.ericsson.otp.erlang.OtpErlangBoolean;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.github.saturn_xiv.palm.hibiscus.otp_node.Handler;

import jakarta.annotation.Resource;

@Component("otp-node.h.argon2")
public class Argon2Handler extends Handler {

    @Override
    public String mailbox() {
        return "argon2";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action, int len) {
        if (action.equals("encode") && len == 3 && request.elementAt(2) instanceof OtpErlangBinary password) {
            final var hashed = passwordEncoder.encode(binary2string(password));
            super.send(mbox, fromPid, new OtpErlangObject[] {
                    super.ok(),
                    new OtpErlangString(hashed)
            });
            return;
        }
        if (action.equals("verify") && len == 4 && request.elementAt(2) instanceof OtpErlangBinary password
                && request.elementAt(3) instanceof OtpErlangString hashed) {
            final var ok = passwordEncoder.matches(binary2string(password), hashed.stringValue());
            super.send(mbox, fromPid, new OtpErlangObject[] {
                    super.ok(),
                    new OtpErlangBoolean(ok)
            });
            return;
        }
        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Resource
    PasswordEncoder passwordEncoder;
    private static final Logger logger = LoggerFactory.getLogger(Argon2Handler.class);
}
