package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;

public record RefreshResponse(PageLayout page, CurrentUser user) implements Serializable {

}
