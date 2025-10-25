package com.github.saturn_xiv.palm;

import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;

public class CamelliaApplication extends Application<CamelliaConfiguration> {

    public static void main(final String[] args) throws Exception {
        new CamelliaApplication().run(args);
    }

    @Override
    public String getName() {
        return "Camellia";
    }

    @Override
    public void initialize(final Bootstrap<CamelliaConfiguration> bootstrap) {
        // TODO: application initialization
    }

    @Override
    public void run(final CamelliaConfiguration configuration,
                    final Environment environment) {
        // TODO: implement application
    }

}
