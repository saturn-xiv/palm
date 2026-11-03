package com.github.saturn_xiv.palm.hibiscus.otp_node;

import java.nio.charset.StandardCharsets;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.scheduling.annotation.Async;

import com.ericsson.otp.erlang.OtpErlangAtom;
import com.ericsson.otp.erlang.OtpErlangBinary;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;

public abstract class Handler {
    public abstract String mailbox();

    protected abstract void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action,
            int len);

    @Async
    public void execute(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action,
            int len) {
        try {
            logger.info("{}@{}/{} {}({})", fromPid.id(), fromPid.node(), mbox.getName(), action, len);
            this.handle(mbox, fromPid, request, action, len);
        } catch (Exception e) {
            logger.error("Internal server error", e);
            this.internalServerError(mbox, fromPid);
        }
    }

    protected OtpErlangAtom ok() {
        return new OtpErlangAtom("ok");
    }

    protected void unimplemented(OtpMbox mbox, OtpErlangPid fromPid) {
        this.abort(mbox, fromPid, "Unimplemented");
    }

    protected void internalServerError(OtpMbox mbox, OtpErlangPid fromPid) {
        this.abort(mbox, fromPid, "Internal Server Error");
    }

    protected void badRequest(OtpMbox mbox, OtpErlangPid fromPid) {
        this.abort(mbox, fromPid, "Bad request");
    }

    protected void abort(OtpMbox mbox, OtpErlangPid fromPid, String reason) {
        this.send(mbox, fromPid, new OtpErlangObject[] {
                new OtpErlangAtom("error"),
                new OtpErlangString(reason)
        });
    }

    protected void send(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangObject[] response) {
        mbox.send(fromPid, new OtpErlangTuple(response));
    }

    protected String binary2string(OtpErlangBinary b) {
        return new String(b.binaryValue(), StandardCharsets.UTF_8);
    }

    private static final Logger logger = LoggerFactory.getLogger(Handler.class);
}
