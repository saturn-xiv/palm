package com.github.saturn_xiv.palm.hyacinth;

import java.io.Serializable;
import java.util.HashMap;
import java.util.Map;

public final class Config implements Serializable {
    public Config() {
        this.nodes = new HashMap<>();
    }

    public final static class Node {
        public String host;
        public int port;
    }

    public Map<String, Node> nodes;
}
