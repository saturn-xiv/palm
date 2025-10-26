package com.github.saturn_xiv.palm.camellia.controllers;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

import jakarta.annotation.Resource;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.dao.DataAccessException;
import org.springframework.graphql.data.method.annotation.QueryMapping;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Controller;

import com.github.saturn_xiv.palm.camellia.responses.HealthCheckResponseItem;

@Controller("palm.camellia.health-check-controller")
public class HealthCheckController {
    @QueryMapping
    public List<HealthCheckResponseItem> healthCheck() {
        List<HealthCheckResponseItem> items = new ArrayList<>();
        items.add(database());
        return items;
    }

    @QueryMapping
    public String version() {
        return getClass().getPackage().getImplementationVersion();
    }

    private HealthCheckResponseItem database() {
        final var name = "database";
        try {
            var version = jdbcTemplate.queryForObject(selectDatabaseVersion(), String.class);
            return new HealthCheckResponseItem(name, Optional.of(version));
        } catch (DataAccessException e) {
            logger.error("select version", e);
            return new HealthCheckResponseItem(name, Optional.empty());
        }
    }

    private String selectDatabaseVersion() {
        switch (databaseDriver) {
            case "org.sqlite.JDBC":
                return "SELECT SQLITE_VERSION()";
            default:
                return "SELECT VERSION()";
        }
    }

    @Resource
    JdbcTemplate jdbcTemplate;
    @Value("${spring.datasource.driver-class-name}")
    String databaseDriver;

    private static final Logger logger = LoggerFactory.getLogger(HealthCheckController.class);
}
