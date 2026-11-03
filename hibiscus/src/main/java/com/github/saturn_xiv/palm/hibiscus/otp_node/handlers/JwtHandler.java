package com.github.saturn_xiv.palm.hibiscus.otp_node.handlers;

import java.security.GeneralSecurityException;
import java.util.ArrayList;
import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangBinary;
import com.ericsson.otp.erlang.OtpErlangList;
import com.ericsson.otp.erlang.OtpErlangLong;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.github.saturn_xiv.palm.hibiscus.otp_node.Handler;
import com.github.saturn_xiv.palm.hibiscus.utils.Jwt;

import jakarta.annotation.Resource;

@Component("otp-node.h.jwt")
public class JwtHandler extends Handler {

    @Override
    public String mailbox() {
        return "jwt";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action, int len) {
        try {
            if (action.equals("sign") && len == 7 && request.elementAt(2) instanceof OtpErlangBinary issuer
                    && request.elementAt(3) instanceof OtpErlangBinary subject
                    && request.elementAt(4) instanceof OtpErlangList audiences
                    && request.elementAt(5) instanceof OtpErlangBinary payload
                    && request.elementAt(6) instanceof OtpErlangLong ttl) {
                List<String> audiences_ = new ArrayList<>();
                for (var it : audiences.elements()) {
                    if (it instanceof OtpErlangBinary audience) {
                        audiences_.add(binary2string(audience));
                        continue;
                    }
                    super.badRequest(mbox, fromPid);
                    return;
                }
                final var token = this.jwt.sign(binary2string(issuer), binary2string(subject), audiences_,
                        binary2string(payload), ttl.longValue());
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok(),
                        new OtpErlangString(token),
                });
                return;
            }
            if (action.equals("verify") && len == 5 && request.elementAt(2) instanceof OtpErlangBinary token
                    && request.elementAt(3) instanceof OtpErlangBinary issuer
                    && request.elementAt(4) instanceof OtpErlangBinary audience) {
                final var response = this.jwt.verify(binary2string(token), binary2string(issuer),
                        binary2string(audience));
                super.send(mbox, fromPid, new OtpErlangObject[] {
                        super.ok(),
                        new OtpErlangString(response.getKey()),
                        new OtpErlangString(response.getValue())
                });
                return;
            }
        } catch (GeneralSecurityException e) {
            logger.error("jwt", e);
            super.internalServerError(mbox, fromPid);
            return;
        }

        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Resource
    Jwt jwt;
    private static final Logger logger = LoggerFactory.getLogger(JwtHandler.class);

}
