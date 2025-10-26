package com.github.saturn_xiv.palm.camellia.services.impl;

import java.util.ArrayList;
import java.util.List;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpErlangException;
import com.ericsson.otp.erlang.OtpErlangList;
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

        if ("all-roles".equals(action)) {
            List<OtpErlangObject> roles = new ArrayList<>();
            for (var it : policyHelper.roles()) {
                OtpErlangObject role = new OtpErlangString(it);
                roles.add(role);
            }
            OtpErlangList list = new OtpErlangList(roles.stream().toArray(OtpErlangObject[]::new));
            items.add(list);
            return items;
        }
        throw new OtpErlangException(String.format("unsupported action %s", action));
    }

    @Resource
    PolicyHelper policyHelper;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangAclServerImpl.class);

}
