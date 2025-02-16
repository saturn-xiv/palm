package com.github.saturn_siv.palm.dahlia.config

import org.casbin.adapter.JDBCAdapter
import org.casbin.jcasbin.main.Enforcer
import org.casbin.jcasbin.main.SyncedEnforcer
import org.casbin.jcasbin.model.Model
import org.casbin.watcher.RedisWatcher
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.beans.factory.annotation.Value
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import javax.sql.DataSource

@Configuration
class CasbinConfiguration {
    @Bean("dahlia.casbin-enforcer")
    fun enforcer(): Enforcer {
        val model =
            Model.newModelFromString(this::class.java.classLoader.getResource("/casbin/rbac_model.conf")?.readText());
        val adapter = JDBCAdapter(datasource)
        val watcher = RedisWatcher(redisHost, redisPort.toInt(), "casbin.topic")
        val enforcer = SyncedEnforcer(model, adapter)
        enforcer.setWatcher(watcher);
        enforcer.loadPolicy();
        return enforcer;
    }

    @Autowired
    private lateinit var datasource: DataSource

    @Value("\${spring.data.redis.host}")
    private lateinit var redisHost: String

    @Value("\${spring.data.redis.port}")
    private lateinit var redisPort: Number
}