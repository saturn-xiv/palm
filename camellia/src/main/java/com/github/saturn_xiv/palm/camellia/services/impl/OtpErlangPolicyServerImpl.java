package com.github.saturn_xiv.palm.camellia.services.impl;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;

import com.github.saturn_xiv.palm.camellia.helpers.PolicyHelper;
import com.github.saturn_xiv.palm.camellia.services.OtpErlangServer;

@Component("palm.camellia.otp-erlang-policy-server")
public class OtpErlangPolicyServerImpl extends OtpErlangServer {

    @Override
    protected List<OtpErlangObject> handle(OtpErlangPid from, OtpErlangTuple request) throws OtpException {
        List<OtpErlangObject> items = new ArrayList<>();

        String name = ((OtpErlangString) request.elementAt(1)).stringValue();
        logger.debug("receive message({}) from {}@{}", name, from.serial(), from.node());
        String greeting = String.format("Hello, %s!", name);
        items.add(new OtpErlangString(greeting));
        return items;
    }

    @PostConstruct
    void init() throws IOException {
        super.launch(nodeName, "policy", cookie);
    }

    @Value("${opt-erlang.node-name}")
    String nodeName;
    @Value("${opt-erlang.cookie}")
    String cookie;
    @Resource
    PolicyHelper policyHelper;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangPolicyServerImpl.class);

}
