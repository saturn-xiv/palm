package com.github.saturn_xiv.palm.hibiscus.otp_node;

import java.io.IOException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Component;

import com.ericsson.otp.erlang.OtpErlangAtom;
import com.ericsson.otp.erlang.OtpErlangDecodeException;
import com.ericsson.otp.erlang.OtpErlangExit;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;

@Component("otp-node.listener")
public class Listener {
    @Async
    public void startUp(OtpNode node, Handler handler) throws IOException {
        final var mailbox = handler.mailbox();
        logger.info("Waiting messages for mailbox {}", mailbox);
        OtpMbox mbox = node.createMbox(mailbox);

        while (!Thread.currentThread().isInterrupted()) {
            try {
                if (mbox.receive() instanceof OtpErlangTuple request) {
                    final var len = request.arity();
                    if (len >= 2 && request.elementAt(0) instanceof OtpErlangPid fromPid
                            && request.elementAt(1) instanceof OtpErlangAtom action) {
                        handler.execute(mbox, fromPid, request, action.atomValue(), len);
                    } else {
                        logger.error("Invalid tuple request");
                    }
                } else {
                    logger.error("Not a tuple request");
                }
            } catch (OtpErlangDecodeException e) {
                logger.error("decode message", e);
            } catch (OtpErlangExit e) {
                logger.error("otp-erlang exit", e);
                break;
            }
        }
    }

    private static final Logger logger = LoggerFactory.getLogger(Listener.class);
}
