package com.github.saturn_xiv.palm.hyacinth;

import java.io.File;
import java.io.IOException;

import org.slf4j.LoggerFactory;
import org.slf4j.Logger;
import com.fasterxml.jackson.dataformat.toml.TomlMapper;

import com.github.saturn_xiv.palm.hyacinth.models.Config;

public class App {
    public static void main(String[] args) throws IOException {
        TomlMapper mapper = new TomlMapper();
        String config_file = "config.toml";
        logger.info("load configuration from {}", config_file);
        Config config = mapper.readValue(new File(config_file), Config.class);
        for (var entry : config.backends().entrySet()) {
            logger.debug("found rpc {} backend http://{}:{}", entry.getKey(), entry.getValue().host(),
                    entry.getValue().port());
        }

    }

    private static final Logger logger = LoggerFactory.getLogger(App.class);
}
