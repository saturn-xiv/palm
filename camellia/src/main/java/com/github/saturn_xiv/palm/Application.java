package com.github.saturn_xiv.palm;

import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;

public class Application extends io.dropwizard.core.Application<Configuration> {

    public static void main(final String[] args) throws Exception {
        new Application().run(args);
    }

    @Override
    public String getName() {
        return "Camellia";
    }

    @Override
    public void initialize(final Bootstrap<Configuration> bootstrap) {
        // TODO: application initialization
    }

    @Override
    public void run(final Configuration configuration,
            final Environment environment) {
        // TODO: implement application
    }

}
