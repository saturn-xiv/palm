package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models;

import java.io.Serializable;
import java.util.List;

public record JobConfig(String version, String summary, String script, List<Arg> args)
        implements Serializable {

    public record Arg(String name, String label, Type type) implements Serializable {
    }

    public enum Type {
        Text, GithubCommitId
    }
}
