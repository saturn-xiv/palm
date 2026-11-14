package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Set;

import org.apache.commons.io.FilenameUtils;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.ImmutableTriple;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.ClassPathResource;
import org.springframework.core.io.Resource;
import org.springframework.stereotype.Component;
import org.yaml.snakeyaml.Yaml;
import org.yaml.snakeyaml.nodes.MappingNode;
import org.yaml.snakeyaml.nodes.NodeTuple;
import org.yaml.snakeyaml.nodes.ScalarNode;

@Component("portal.u.locales-sync")
public class LocalesSync {

    public List<ImmutableTriple<Locale, String, String>> load() throws IOException {
        List<ImmutableTriple<Locale, String, String>> items = new ArrayList<>();

        final var root = Path.of("locales");
        List<Resource> global = new ArrayList<>();
        List<ImmutablePair<Locale, Resource>> files = new ArrayList<>();
        Set<Locale> languages = new HashSet<>();
        for (var it : this.resources) {
            if (it.isReadable()) {
                if (it instanceof ClassPathResource res) {
                    var path = res.getPath();
                    var parent = Path.of(path).getParent().getFileName();
                    if (parent.equals(root)) {
                        global.add(it);
                    } else {
                        final var lang = Locale.forLanguageTag(parent.getFileName().toString());
                        languages.add(lang);
                        files.add(ImmutablePair.of(lang, it));
                    }
                }
            }
        }

        for (var lang : languages) {
            for (var file : global) {
                items.addAll(load(lang, file));
            }
        }
        for (var it : files) {
            items.addAll(load(it.left, it.right));
        }

        return items;
    }

    private static List<ImmutableTriple<Locale, String, String>> load(Locale lang, Resource file) throws IOException {
        logger.info("load from {} for language {}", file.getURL(), lang.toLanguageTag());

        if (file instanceof ClassPathResource res) {

            final var zone = FilenameUtils.removeExtension(res.getFilename());
            try (InputStream ins = file.getInputStream();
                    BufferedReader reader = new BufferedReader(new InputStreamReader(ins, StandardCharsets.UTF_8));) {
                var yaml = new Yaml();
                var root = yaml.compose(reader);
                if (root instanceof MappingNode node) {
                    return load(lang, zone, node);
                }
            }
        }
        return new ArrayList<>();
    }

    private static List<ImmutableTriple<Locale, String, String>> load(Locale lang, String namespace, MappingNode map) {
        List<ImmutableTriple<Locale, String, String>> items = new ArrayList<>();

        for (NodeTuple tuple : map.getValue()) {
            ScalarNode key = (ScalarNode) tuple.getKeyNode();
            String code = String.format("%s.%s", namespace, key.getValue());
            var value = tuple.getValueNode();
            if (value instanceof MappingNode node) {
                items.addAll(load(lang, code, node));
            } else if (value instanceof ScalarNode node) {
                items.add(ImmutableTriple.of(lang, code, node.getValue()));
            } else {
                logger.error("ignore node {}", code);
            }
        }

        return items;
    }

    @Value("classpath:locales/**/*.yml")
    private Resource[] resources;
    private final static Logger logger = LoggerFactory.getLogger(LocalesSync.class);
}
