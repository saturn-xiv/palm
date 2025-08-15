package com.github.saturn_xiv.palm.hyacinth;

import java.io.File;
import java.io.IOException;

import com.fasterxml.jackson.dataformat.toml.TomlMapper;

import com.github.saturn_xiv.palm.hyacinth.models.Config;

public class App {
    public static void main(String[] args) throws IOException {
        TomlMapper mapper = new TomlMapper();
        Config config = mapper.readValue(new File("config.toml"), Config.class);
        System.out.println("Hello World!" + config.backend().host());
    }
}
