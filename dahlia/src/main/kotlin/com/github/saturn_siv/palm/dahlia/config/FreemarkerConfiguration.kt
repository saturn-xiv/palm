package com.github.saturn_siv.palm.dahlia.config

import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Component
import javax.annotation.PostConstruct

@Component("dahlia.freemarker-setup")
class FreemarkerConfiguration {
    @PostConstruct
    fun init() {
// TODO global var
        this.configuration.setSharedVariable("siteTitle", "ttt");
    }

    @Autowired
    private lateinit var configuration: freemarker.template.Configuration;
}