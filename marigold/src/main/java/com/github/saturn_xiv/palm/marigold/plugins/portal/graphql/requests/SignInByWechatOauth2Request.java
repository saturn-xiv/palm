package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests;

import java.io.Serializable;

public record SignInByWechatOauth2Request(String state, String code, String lang, String timezone)
                implements Serializable {

}
