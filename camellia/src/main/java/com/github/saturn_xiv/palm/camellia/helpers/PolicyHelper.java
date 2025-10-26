package com.github.saturn_xiv.palm.camellia.helpers;

import java.io.InputStreamReader;
import javax.sql.DataSource;

import jakarta.annotation.PostConstruct;
import jakarta.annotation.Resource;

import org.casbin.adapter.JDBCAdapter;
import org.casbin.jcasbin.main.Enforcer;
import org.casbin.jcasbin.model.Model;
import org.casbin.jcasbin.persist.Adapter;
import org.casbin.jcasbin.persist.Watcher;
import org.casbin.watcher.lettuce.LettuceRedisWatcher;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.ResourceLoader;
import org.springframework.stereotype.Component;
import org.springframework.util.FileCopyUtils;

@Component("palm.camellia.policy-helper")
public class PolicyHelper {

    @PostConstruct
    void init() throws Exception {
        Model model = new Model();
        {
            var file = String.format("classpath:casbin/%s_model.conf", modelName);
            logger.debug("load casbin model from {}", file);
            var resource = resourceLoader.getResource(file);
            try (var reader = new InputStreamReader(resource.getInputStream())) {
                var content = FileCopyUtils.copyToString(reader);
                model.loadModelFromText(content);
            }
        }
        Adapter adapter = new JDBCAdapter(dataSource);
        Watcher watcher = new LettuceRedisWatcher(redisHost, redisPort,
                String.format("%s://casbin.watcher", applicationName));
        enforcer = new Enforcer(model, adapter, true);
        enforcer.setWatcher(watcher);

        enforcer.loadPolicy();
    }

    @Resource
    DataSource dataSource;
    @Resource
    ResourceLoader resourceLoader;
    // https://github.com/casbin/casbin/tree/master/examples
    @Value("${camellia.casbin.model}")
    String modelName;
    @Value("${spring.data.redis.host}")
    String redisHost;
    @Value("${spring.data.redis.port}")
    int redisPort;
    @Value("${spring.application.name}")
    String applicationName;

    private Enforcer enforcer;
    private static final Logger logger = LoggerFactory.getLogger(PolicyHelper.class);
}
