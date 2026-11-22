package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests;

import java.io.Serializable;

public record GoogleOauth2RevokeRequest(String token) implements Serializable {

}
