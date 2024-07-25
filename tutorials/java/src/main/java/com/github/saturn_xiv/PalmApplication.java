package com.github.saturn_xiv;

import com.github.saturn_xiv.palm.plugins.balsam.v1.HMacGrpc;
import com.github.saturn_xiv.palm.plugins.balsam.v1.HMacSignRequest;
import com.github.saturn_xiv.palm.plugins.balsam.v1.HMacVerifyRequest;
import com.google.protobuf.ByteString;
import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class PalmApplication extends Application<PalmConfiguration> {

    public static void main(final String[] args) throws Exception {
        new PalmApplication().run(args);
    }

    @Override
    public String getName() {
        return "Palm";
    }

    @Override
    public void initialize(final Bootstrap<PalmConfiguration> bootstrap) {
        // TODO: application initialization
    }

    @Override
    public void run(final PalmConfiguration configuration,
                    final Environment environment) throws IOException {
        var atropaChannel = configuration.getAtropa().Open();
        {
            logger.debug("test atropa.hmac service");
            final var hi = "Hello, atropa!";
            var stub = HMacGrpc.newBlockingStub(atropaChannel);
            var res = stub.sign(HMacSignRequest.newBuilder().setPlain(ByteString.copyFromUtf8(hi)).build());
            logger.info("hmac sign({}): ({})", hi, res.getCode());
            var empty = stub.verify(HMacVerifyRequest.newBuilder().setCode(res.getCode()).setPlain(ByteString.copyFromUtf8(hi)).build());
        }
    }

    private final static Logger logger = LoggerFactory.getLogger(PalmApplication.class);

}
