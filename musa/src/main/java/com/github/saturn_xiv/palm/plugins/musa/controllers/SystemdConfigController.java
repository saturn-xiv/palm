package com.github.saturn_xiv.palm.plugins.musa.controllers;

import com.github.saturn_xiv.palm.plugins.musa.models.SystemdConfig;
import com.samskivert.mustache.Mustache;
import jakarta.validation.constraints.NotBlank;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.io.ResourceLoader;
import org.springframework.core.io.support.PropertiesLoaderUtils;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.io.IOException;

@RestController("palm.musa.controller.systemd-conf")
@RequestMapping("/api")
@Validated
public class SystemdConfigController {
    @GetMapping("/systemd.conf")
    public String get(@NotBlank @RequestParam("name") String name) throws IOException {
        final var resource = resourceLoader.getResource("git.properties");
        final var props = PropertiesLoaderUtils.loadProperties(resource);
        final var version = props.getProperty("git.build.version");
        return mustacheCompiler.loadTemplate("systemd-config").execute(new SystemdConfig(name, version));
    }

    @Autowired
    Mustache.Compiler mustacheCompiler;
    @Autowired
    ResourceLoader resourceLoader;
}
