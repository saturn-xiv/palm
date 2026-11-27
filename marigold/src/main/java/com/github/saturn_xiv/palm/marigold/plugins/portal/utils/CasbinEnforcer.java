package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.function.Function;

import javax.sql.DataSource;

import org.apache.commons.lang3.tuple.Pair;
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

import com.github.saturn_xiv.palm.plugins.rbac.v1.Action;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Object;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Permission;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.User;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.Administrator;
import com.github.saturn_xiv.palm.plugins.rbac.v1.Subject.Role.Root;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.Message;
import com.google.protobuf.Parser;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

@Component("portal.u.casbin-enforcer")
public class CasbinEnforcer {

    public <T> T execute(Function<Pair<Enforcer, Watcher>, T> callback) {
        return callback.apply(Pair.of(enforcer, watcher));
    }

    public void addRoleForUser(int user, Role role) {
        if (enforcer.addRoleForUser(subject(user), subject(role))) {
            watcher.update();
        }
    }

    public void deleteRoleForUser(int user, Role role) {
        if (enforcer.deleteRoleForUser(subject(user), subject(role))) {
            watcher.update();
        }
    }

    public List<Permission> getImplicitPermissionsForUser(int user) throws InvalidProtocolBufferException {
        List<Permission> items = new ArrayList<>();
        for (var rule : enforcer.getImplicitPermissionsForUser(subject(user))) {
            if (rule.size() != 3) {
                logger.error("invalid permission {}", String.join(",", rule));
                continue;
            }
            items.add(Permission.newBuilder()
                    .setAction(string2message(rule.get(1), Action.parser()))
                    .setObject(string2message(rule.get(2), Object.parser()))
                    .build());
        }
        return items;
    }

    public List<Role> getImplicitRolesForUser(int user) throws InvalidProtocolBufferException {
        List<Role> items = new ArrayList<>();
        for (var it : enforcer.getImplicitRolesForUser(subject(user))) {
            var subject = string2message(it, Subject.parser());
            if (subject.hasRole()) {
                items.add(subject.getRole());
            }
        }
        return items;
    }

    public List<Role> getRolesForUser(int user) throws InvalidProtocolBufferException {
        List<Role> items = new ArrayList<>();
        for (var it : enforcer.getRolesForUser(subject(user))) {
            var subject = string2message(it, Subject.parser());
            if (subject.hasRole()) {
                items.add(subject.getRole());
            }
        }
        return items;
    }

    public boolean has(int user, Role role) {
        final var roles = enforcer.getImplicitRolesForUser(subject(user));
        return roles.contains(subject(role));
    }

    public static Role administrator() {
        return Role.newBuilder().setAdministrator(Administrator.newBuilder().build()).build();
    }

    public static Role root() {
        return Role.newBuilder().setRoot(Root.newBuilder().build()).build();
    }

    public static Role role(String code) {
        return Role.newBuilder().setCode(code).build();
    }

    public static String subject(int user) {
        var it = Subject.newBuilder().setUser(User.newBuilder().setId(user).build()).build();
        return message2string(it);
    }

    public static String subject(Role role) {
        var it = Subject.newBuilder().setRole(role).build();
        return message2string(it);
    }

    public static <T extends Message> String message2string(T message) {
        final var buf = message.toByteArray();
        return Base64.getUrlEncoder().withoutPadding().encodeToString(buf);
    }

    public static <T extends Message> T string2message(String message, Parser<T> clazz)
            throws InvalidProtocolBufferException {
        final var buf = Base64.getUrlDecoder().decode(message);
        return clazz.parseFrom(buf);
    }

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
                        rabbitmqVirtualHost),
                "marigold.casbin");
        enforcer = new Enforcer(model, adapter);

        watcher.setUpdateCallback(() -> {
            logger.info("Policy update received, reloading...");
            enforcer.loadPolicy();
        });
        enforcer.setWatcher(watcher);
        enforcer.loadPolicy();
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
