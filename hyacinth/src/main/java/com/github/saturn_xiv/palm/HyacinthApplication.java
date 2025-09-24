package com.github.saturn_xiv.palm;

import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;

public class HyacinthApplication extends Application<HyacinthConfiguration> {

    public static void main(final String[] args) throws Exception {
        new HyacinthApplication().run(args);
    }

    @Override
    public String getName() {
        return "Hyacinth";
    }

    @Override
    public void initialize(final Bootstrap<HyacinthConfiguration> bootstrap) {
        // TODO: application initialization
    }

    @Override
    public void run(final HyacinthConfiguration configuration,
                    final Environment environment) {
        // TODO: implement application
    }

}
