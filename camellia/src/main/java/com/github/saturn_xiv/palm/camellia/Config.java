package com.github.saturn_xiv.palm.camellia;

import java.io.Serializable;
import java.util.Map;

public final class Config implements Serializable {
    public final class Node {
        public String host;
        public int port;
    }

    public Map<String, Node> nodes;
}
