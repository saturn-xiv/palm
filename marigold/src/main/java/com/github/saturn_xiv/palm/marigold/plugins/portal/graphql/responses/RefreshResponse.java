package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;

public record RefreshResponse(Layout page, CurrentUser user) implements Serializable {

}
