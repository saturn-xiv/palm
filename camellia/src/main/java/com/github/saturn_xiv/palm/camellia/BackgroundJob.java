package com.github.saturn_xiv.palm.camellia;

import java.io.IOException;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.task.TaskExecutor;
import org.springframework.stereotype.Component;
import com.ericsson.otp.erlang.OtpNode;

import com.github.saturn_xiv.palm.camellia.services.OtpErlangRunnableServer;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangAesServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangDocumentServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangHMacServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangJwtServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangMailServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangAclServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangS3ServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangTwilioServerImpl;
import com.github.saturn_xiv.palm.camellia.services.impl.OtpErlangWechatServerImpl;

@Component("palm.camellia.background-jobs")
public class BackgroundJob {

    @PostConstruct
    void init() throws IOException {
        OtpNode node = new OtpNode(optErlangNodeName, optErlangCookie);
        taskExecutor.execute(new OtpErlangRunnableServer(aesServer, node, "aes"));
        taskExecutor.execute(new OtpErlangRunnableServer(hmacServer, node, "hmac"));
        taskExecutor.execute(new OtpErlangRunnableServer(jwtServer, node, "jwt"));
        taskExecutor.execute(new OtpErlangRunnableServer(mailServer, node, "mail"));
        taskExecutor.execute(new OtpErlangRunnableServer(twilioServer, node, "twilio"));
        taskExecutor.execute(new OtpErlangRunnableServer(s3Server, node, "s3"));
        taskExecutor.execute(new OtpErlangRunnableServer(wechatServer, node, "wechat"));
        taskExecutor.execute(new OtpErlangRunnableServer(aclServer, node, "acl"));
        taskExecutor.execute(new OtpErlangRunnableServer(documentServer, node, "doc"));
    }

    @Resource
    TaskExecutor taskExecutor;

    @Resource
    OtpErlangAesServerImpl aesServer;
    @Resource
    OtpErlangHMacServerImpl hmacServer;
    @Resource
    OtpErlangJwtServerImpl jwtServer;
    @Resource
    OtpErlangMailServerImpl mailServer;
    @Resource
    OtpErlangTwilioServerImpl twilioServer;
    @Resource
    OtpErlangS3ServerImpl s3Server;
    @Resource
    OtpErlangWechatServerImpl wechatServer;
    @Resource
    OtpErlangAclServerImpl aclServer;
    @Resource
    OtpErlangDocumentServerImpl documentServer;

    @Value("${opt-erlang.node-name}")
    String optErlangNodeName;
    @Value("${opt-erlang.cookie}")
    String optErlangCookie;

}
