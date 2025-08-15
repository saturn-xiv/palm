package com.github.saturn_xiv.palm.hyacinth;

import java.io.File;
import java.io.IOException;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.dataformat.toml.TomlMapper;

import com.github.saturn_xiv.palm.hyacinth.models.Backend;
import com.github.saturn_xiv.palm.hyacinth.models.Config;

public class Server {
    public Server(String config_file) throws IOException {
        TomlMapper mapper = new TomlMapper();
        logger.info("load configuration from {}", config_file);
        Config config = mapper.readValue(new File(config_file), Config.class);
        for (var entry : config.backends().entrySet()) {
            logger.debug("found rpc {} backend tcp://{}:{}", entry.getKey(), entry.getValue().host(),
                    entry.getValue().port());
        }
    }

    public void launch(int port) {
        logger.info("listen on http://0.0.0.0:{}", port);
    }

    private Map<String, Backend> backends;

    private static final Logger logger = LoggerFactory.getLogger(Server.class);
}
