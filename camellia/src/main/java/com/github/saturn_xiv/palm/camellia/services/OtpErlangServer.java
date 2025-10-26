package com.github.saturn_xiv.palm.camellia.services;

import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.ericsson.otp.erlang.OtpErlangAtom;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpException;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;

public abstract class OtpErlangServer {
    public void launch(OtpNode node, String mailboxName) {
        logger.info("start opt-erlang server {}://{}", node.node(), mailboxName);
        OtpMbox box = node.createMbox(mailboxName);
        while (true) {
            try {
                this.handle(node, box);
            } catch (OtpException e) {
                logger.error("message handler", e);
            }
        }
    }

    private void handle(OtpNode node, OtpMbox box) throws OtpException {
        OtpErlangObject object = box.receive();
        if (object instanceof OtpErlangTuple) {
            OtpErlangTuple request = (OtpErlangTuple) object;
            if (request.elements().length < 1) {
                logger.error("incomplete message payload");
                return;
            }

            OtpErlangPid from = (OtpErlangPid) request.elementAt(0);
            String action = ((OtpErlangAtom) request.elementAt(1)).atomValue();
            logger.debug("receive message ({}) from {}://{}", action, from.serial(), from.node());
            var response = this.handle(from, action, request);
            response.add(0, box.self());
            OtpErlangTuple reply = new OtpErlangTuple(response.stream().toArray(OtpErlangObject[]::new));
            box.send(from, reply);

            return;
        }
        logger.error("unsupported message type");
    }

    protected abstract List<OtpErlangObject> handle(OtpErlangPid from, String action, OtpErlangTuple request)
            throws OtpException;

    private static final Logger logger = LoggerFactory.getLogger(OtpErlangServer.class);
}
