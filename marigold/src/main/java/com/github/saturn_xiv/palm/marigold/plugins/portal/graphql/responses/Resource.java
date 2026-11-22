package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;
import java.util.Optional;

public record Resource(String type, Optional<Integer> id) implements Serializable {

}
