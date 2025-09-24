package com.github.saturn_xiv.palm.hyacinth.models;

import java.io.IOException;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.google.common.reflect.ClassPath;

public record Config(Set<String> packages, Map<String, Backend> backends) {

    public void load() throws IOException, ClassNotFoundException {
        for (var pkg : this.packages()) {
            @SuppressWarnings("rawtypes")
            Set<Class> classes = ClassPath.from(ClassLoader.getSystemClassLoader())
                    .getAllClasses()
                    .stream()
                    .filter(it -> it.getPackageName().startsWith(pkg))
                    .map(it -> it.load())
                    .collect(Collectors.toSet());
            logger.debug("found {} classes in package {}", classes.size(), pkg);
        }
    }

    private static final Logger logger = LoggerFactory.getLogger(Config.class);
}
