package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse;

public class SignInResponse implements Serializable {
    public SignInResponse(Item user, String sn, UserIndexResponse.Item.Type type) {
    }

}
