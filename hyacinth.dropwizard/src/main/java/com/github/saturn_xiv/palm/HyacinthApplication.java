package com.github.saturn_xiv.palm;

import jakarta.inject.Singleton;

import org.glassfish.hk2.api.ServiceLocator;
import org.glassfish.hk2.utilities.binding.AbstractBinder;
import org.glassfish.jersey.servlet.ServletContainer;

import io.dropwizard.core.Application;
import io.dropwizard.core.setup.Bootstrap;
import io.dropwizard.core.setup.Environment;
import io.dropwizard.db.DataSourceFactory;
import io.dropwizard.migrations.MigrationsBundle;

import com.github.saturn_xiv.palm.health.DatabaseHealthCheck;

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
        bootstrap.addBundle(new MigrationsBundle<HyacinthConfiguration>() {
            @Override
            public DataSourceFactory getDataSourceFactory(HyacinthConfiguration configuration) {
                return configuration.getDataSourceFactory();
            }
        });
    }

    @Override
    public void run(final HyacinthConfiguration configuration,
            final Environment environment) {
        environment
                .jersey()
                .register(
                        new AbstractBinder() {
                            @Override
                            protected void configure() {
                                bindAsContract(DatabaseHealthCheck.class).in(Singleton.class);
                            }
                        });

    }

}
