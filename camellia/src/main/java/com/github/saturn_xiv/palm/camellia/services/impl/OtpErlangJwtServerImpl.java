package com.github.saturn_xiv.palm.camellia.services.impl;

import java.io.IOException;
import java.util.List;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;

import com.github.saturn_xiv.palm.camellia.helpers.JwtHelper;
import com.github.saturn_xiv.palm.camellia.services.OtpErlangServer;

@Component("palm.camellia.otp-erlang-jwt-server")
public class OtpErlangJwtServerImpl extends OtpErlangServer {

    @Override
    protected List<OtpErlangObject> handle(OtpErlangPid from, OtpErlangTuple request) throws OtpException {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'handle'");
    }

    @PostConstruct
    void init() throws IOException {
        super.launch(nodeName, "jwt", cookie);
    }

    @Value("${opt-erlang.node-name}")
    String nodeName;
    @Value("${opt-erlang.cookie}")
    String cookie;
    @Resource
    JwtHelper jwtHelper;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangJwtServerImpl.class);
}
