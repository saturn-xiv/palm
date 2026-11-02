package com.github.saturn_xiv.palm.services;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.sql.SQLException;
import java.util.concurrent.TimeoutException;

import javax.sql.DataSource;

import org.casbin.adapter.HibernateAdapter;
import org.casbin.jcasbin.main.Enforcer;
import org.casbin.jcasbin.model.Model;
import org.casbin.jcasbin.persist.Adapter;
import org.casbin.watcher.RabbitMQWatcher;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RbacService {
    public RbacService(DataSource dataSource, String rabbitmqUri) throws SQLException, IOException, TimeoutException {
        InputStream modelStream = getClass().getClassLoader().getResourceAsStream("rbac_model.conf");
        Model model = new Model();
        model.loadModelFromText(new String(modelStream.readAllBytes(), StandardCharsets.UTF_8));

        Adapter adapter = new HibernateAdapter(dataSource);

        this.enforcer = new Enforcer(model, adapter);
        enforcer.setAdapter(adapter);
        this.watcher = new RabbitMQWatcher(rabbitmqUri);
        watcher.setUpdateCallback(() -> {
            logger.info("Policy update received, reloading...");
            enforcer.loadPolicy();
        });
        enforcer.setWatcher(watcher);
        enforcer.loadPolicy();
    }

    public void addRoleForUser(String user, String role) {
        final var ok = this.enforcer.addRoleForUser(user, role);
        if (ok) {
            this.watcher.update();
        }
    }

    private final Enforcer enforcer;
    private final RabbitMQWatcher watcher;
    private static final Logger logger = LoggerFactory.getLogger(RbacService.class);
}
