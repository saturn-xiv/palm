package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests;

import java.io.Serializable;

public record SignInByGoogleOauth2Request(String state, String code, String redirectUri, String lang, String timezone)
        implements Serializable {

}
