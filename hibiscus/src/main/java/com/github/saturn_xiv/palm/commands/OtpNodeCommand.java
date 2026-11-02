package com.github.saturn_xiv.palm.commands;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.ericsson.otp.erlang.OtpErlangAtom;
import com.ericsson.otp.erlang.OtpErlangObject;
import com.ericsson.otp.erlang.OtpErlangPid;
import com.ericsson.otp.erlang.OtpErlangString;
import com.ericsson.otp.erlang.OtpErlangTuple;
import com.ericsson.otp.erlang.OtpMbox;
import com.ericsson.otp.erlang.OtpNode;
import com.github.saturn_xiv.palm.HibiscusConfiguration;

import io.dropwizard.core.cli.ConfiguredCommand;
import io.dropwizard.core.setup.Bootstrap;
import net.sourceforge.argparse4j.inf.Namespace;
import net.sourceforge.argparse4j.inf.Subparser;

public class OtpNodeCommand extends ConfiguredCommand<HibiscusConfiguration> {

    public OtpNodeCommand() {
        super("otp-node", "Launch a Java OTP Node server");
    }

    @Override
    public void configure(Subparser subparser) {
        super.addFileArgument(subparser);
        subparser.addArgument("-n", "--name")
                .dest(NAME)
                .type(String.class)
                .required(true)
                .help("Node name");
        subparser.addArgument("-c", "--cookie")
                .dest(COOKIE)
                .type(String.class)
                .required(true)
                .help("Secret token");
        subparser.addArgument("-m", "--mailbox")
                .dest(MAILBOX)
                .type(String.class)
                .required(true)
                .help("Mailbox");
    }

    @Override
    protected void run(Bootstrap<HibiscusConfiguration> bootstrap, Namespace namespace,
            HibiscusConfiguration configuration)
            throws Exception {
        final var name = namespace.getString(NAME);
        final var cookie = namespace.getString(COOKIE);
        final var mailbox = namespace.getString(MAILBOX);
        logger.info("start java otp server {}://{}", name, mailbox);

        OtpNode node = new OtpNode(name, cookie);
        OtpMbox mbox = node.createMbox(mailbox);
        logger.info("Java Node started, waiting for messages...");

        while (true) {
            final var request = (OtpErlangTuple) mbox.receive();
            final var fromPid = (OtpErlangPid) request.elementAt(0);

            try {
                final var response = this.handle(request);
                mbox.send(fromPid, new OtpErlangTuple(response));
            } catch (Exception e) {
                final var reason = "Internal Server Error";
                logger.error(reason, e);
                mbox.send(fromPid, new OtpErlangTuple(this.abort(reason)));
            }
        }
    }

    private OtpErlangObject[] handle(OtpErlangTuple request) {
        final var action = (OtpErlangAtom) request.elementAt(1);

        if (action.atomValue().equals("version")) {
            logger.info("{}", action);
            final var version = OtpNodeCommand.class.getPackage().getImplementationVersion();
            return new OtpErlangObject[] {
                    new OtpErlangAtom(OK),
                    new OtpErlangString(version)
            };
        }

        final var reason = "Unimplemented";
        logger.error("{} {}", reason, action);
        return abort(reason);
    }

    private OtpErlangObject[] abort(String reason) {
        return new OtpErlangObject[] {
                new OtpErlangAtom(ERROR),
                new OtpErlangString(reason)

        };
    }

    private static final String OK = "ok";
    private static final String ERROR = "error";
    private static final String COOKIE = "cookie";
    private static final String NAME = "name";
    private static final String MAILBOX = "mailbox";
    private static final Logger logger = LoggerFactory.getLogger(OtpNodeCommand.class);
}
