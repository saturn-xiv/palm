package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;

import javax.sql.DataSource;

import org.casbin.adapter.JDBCAdapter;
import org.casbin.jcasbin.main.Enforcer;
import org.casbin.jcasbin.model.Model;
import org.casbin.jcasbin.persist.Adapter;
import org.casbin.jcasbin.persist.Watcher;
import org.casbin.watcher.RabbitMQWatcher;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

@Component("portal.u.casbin-enforcer")
public class CasbinEnforcer {
    @PostConstruct
    void init() throws Exception {
        InputStream modelStream = getClass().getClassLoader().getResourceAsStream("rbac_model.conf");
        final Model model = new Model();
        model.loadModelFromText(new String(modelStream.readAllBytes(), StandardCharsets.UTF_8));

        logger.debug("create jdbc adapter {}", datasourceUrl);
        final Adapter adapter = new JDBCAdapter(datasource);

        logger.debug("create rabbitmq watcher {}@{}:{}/{}", rabbitmqUsername, rabbitmqHost, rabbitmqPort,
                rabbitmqVirtualHost);
        watcher = new RabbitMQWatcher(
                String.format("amqp://%s:%s@%s:%d/%s", rabbitmqUsername, rabbitmqPassword, rabbitmqHost, rabbitmqPort,
                        rabbitmqVirtualHost), "marigold.casbin");
        enforcer = new Enforcer(model, adapter);

        watcher.setUpdateCallback(() -> {
            logger.info("Policy update received, reloading...");
            enforcer.loadPolicy();
        });
        enforcer.setWatcher(watcher);
        enforcer.loadPolicy();
    }

    public void addRoleForUser(String user, String role) {
        if (enforcer.addRoleForUser(user, role)) {
            watcher.update();
        }
    }

    @Resource
    DataSource datasource;
    @Value("${spring.rabbitmq.host}")
    String rabbitmqHost;
    @Value("${spring.rabbitmq.port}")
    int rabbitmqPort;
    @Value("${spring.rabbitmq.username}")
    String rabbitmqUsername;
    @Value("${spring.rabbitmq.password}")
    String rabbitmqPassword;
    @Value("${spring.rabbitmq.virtual-host}")
    String rabbitmqVirtualHost;
    @Value("${spring.datasource.url}")
    String datasourceUrl;

    private Enforcer enforcer;
    private Watcher watcher;

    private static final Logger logger = LoggerFactory.getLogger(CasbinEnforcer.class);
}
