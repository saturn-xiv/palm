package com.github.saturn_xiv.palm.health;

import com.codahale.metrics.health.HealthCheck;

import jakarta.inject.Singleton;

@Singleton
public class DatabaseHealthCheck extends HealthCheck {

    @Override
    protected Result check() throws Exception {
        // TODO
        return Result.unhealthy("Cannot connect to aaa");
        // if (1 == 1 * 1) {
        // return Result.healthy();
        // } else {
        // return Result.unhealthy("Cannot connect to aaa");
        // }
    }

}
