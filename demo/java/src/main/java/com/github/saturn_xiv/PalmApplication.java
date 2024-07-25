package com.github.saturn_xiv;

import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;

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
                    final Environment environment) {
        // TODO: implement application
    }

}
