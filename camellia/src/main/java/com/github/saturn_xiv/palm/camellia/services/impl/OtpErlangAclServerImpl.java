package com.github.saturn_xiv.palm.camellia.services.impl;

import java.util.ArrayList;
import java.util.List;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;

import com.github.saturn_xiv.palm.camellia.helpers.PolicyHelper;
import com.github.saturn_xiv.palm.camellia.services.OtpErlangServer;

@Component("palm.camellia.otp-erlang-acl-server")
public class OtpErlangAclServerImpl extends OtpErlangServer {

    @Override
    protected List<OtpErlangObject> handle(OtpErlangPid from, String action, OtpErlangTuple request)
            throws OtpException {
        List<OtpErlangObject> items = new ArrayList<>();

        String greeting = String.format("Hello, %s!", action);
        items.add(new OtpErlangString(greeting));
        return items;
    }

    @Resource
    PolicyHelper policyHelper;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangAclServerImpl.class);

}
