package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;

public record Permission(String action, Resource resource) implements Serializable {

}
