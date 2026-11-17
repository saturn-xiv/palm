package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;

public record CurrentUser(String name, String avatar) implements Serializable {

}
