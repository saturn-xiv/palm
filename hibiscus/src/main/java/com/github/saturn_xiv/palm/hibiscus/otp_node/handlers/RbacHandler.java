package com.github.saturn_xiv.palm.hibiscus.otp_node.handlers;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.github.saturn_xiv.palm.hibiscus.otp_node.Handler;
import com.github.saturn_xiv.palm.hibiscus.utils.CasbinEnforcer;

import jakarta.annotation.Resource;

@Component("otp-node.h.rbac")
public class RbacHandler extends Handler {

    @Override
    public String mailbox() {
        return "rbac";
    }

    @Override
    protected void handle(OtpMbox mbox, OtpErlangPid fromPid, OtpErlangTuple request, String action, int len) {
        // TODO
        logger.error("Unimplemented action {}", action);
        super.unimplemented(mbox, fromPid);
    }

    @Resource
    CasbinEnforcer casbinEnforcer;

    private static final Logger logger = LoggerFactory.getLogger(RbacHandler.class);
}
