package com.github.saturn_xiv.palm;

import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;

public class HibiscusApplication extends Application<HibiscusConfiguration> {

    public static void main(final String[] args) throws Exception {
        new HibiscusApplication().run(args);
    }

    @Override
    public String getName() {
        return "Hibiscus";
    }

    @Override
    public void initialize(final Bootstrap<HibiscusConfiguration> bootstrap) {
        // TODO: application initialization
    }

    @Override
    public void run(final HibiscusConfiguration configuration,
                    final Environment environment) {
        // TODO: implement application
    }

}
