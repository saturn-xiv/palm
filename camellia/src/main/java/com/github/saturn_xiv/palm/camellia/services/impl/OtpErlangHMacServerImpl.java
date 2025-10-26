package com.github.saturn_xiv.palm.camellia.services.impl;

import java.util.List;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;

import com.github.saturn_xiv.palm.camellia.helpers.HMacHelper;
import com.github.saturn_xiv.palm.camellia.services.OtpErlangServer;

@Component("palm.camellia.otp-erlang-hmac-server")
public class OtpErlangHMacServerImpl extends OtpErlangServer {
    @Override
    protected List<OtpErlangObject> handle(OtpErlangPid from, String action, OtpErlangTuple request)
            throws OtpException {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'handle'");
    }

    @Resource
    HMacHelper hmacHelper;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangHMacServerImpl.class);
}
