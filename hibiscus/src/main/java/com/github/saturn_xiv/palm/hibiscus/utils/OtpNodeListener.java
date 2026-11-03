package com.github.saturn_xiv.palm.hibiscus.utils;

import java.io.IOException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangAtom;
import com.ericsson.otp.erlang.OtpErlangDecodeException;
import com.ericsson.otp.erlang.OtpErlangExit;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;

import jakarta.annotation.Resource;

@Component("u.otp-node-listener")
public class OtpNodeListener {
    @Async
    public void startUp() throws IOException {
        OtpNode node = new OtpNode(name, cookie);
        OtpMbox mbox = node.createMbox(mailbox);
        logger.info("Java otp node started at {}://{}, waiting for messages...", name, mailbox);
        while (!Thread.currentThread().isInterrupted()) {
            try {
                if (mbox.receive() instanceof OtpErlangTuple request) {
                    final var len = request.arity();
                    if (len >= 2 && request.elementAt(0) instanceof OtpErlangPid fromPid
                            && request.elementAt(1) instanceof OtpErlangAtom action) {
                        try {
                            final var response = this.handle(request, action.atomValue(), len);
                            mbox.send(fromPid, new OtpErlangTuple(response));
                        } catch (Exception e) {
                            final var reason = "Internal Server Error";
                            logger.error(reason, e);
                            mbox.send(fromPid, new OtpErlangTuple(this.fail(reason)));
                        }
                    } else {
                        logger.error("Invalid tuple data");
                    }
                } else {
                    logger.error("Not a tuple data");
                }
            } catch (OtpErlangDecodeException e) {
                logger.error("decode message", e);
            } catch (OtpErlangExit e) {
                break;
            }
        }
    }

    private OtpErlangObject[] handle(OtpErlangTuple request, String action, int len) {
        if (action.equals("version") && len == 2) {
            logger.info("{}", action);
            return new OtpErlangObject[] {
                    new OtpErlangAtom(OK),
                    new OtpErlangString(version)
            };
        }

        final var reason = "Unimplemented";
        logger.error("{} {}", reason, action);
        return fail(reason);
    }

    private OtpErlangObject[] fail(String reason) {
        return new OtpErlangObject[] {
                new OtpErlangAtom(ERROR),
                new OtpErlangString(reason)
        };
    }

    @Resource
    CasbinEnforcer casbinEnforcer;

    @Value("${hibiscus.otp-node.name}")
    String name;
    @Value("${hibiscus.otp-node.cookie}")
    String cookie;
    @Value("${hibiscus.otp-node.mailbox}")
    String mailbox;
    @Value("${hibiscus.version}")
    String version;

    private static final String OK = "ok";
    private static final String ERROR = "error";

    private static final Logger logger = LoggerFactory.getLogger(OtpNodeListener.class);
}
