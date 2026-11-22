package com.github.saturn_xiv.palm.marigold.plugins.portal.models;

import java.io.Serializable;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;
import com.github.saturn_xiv.palm.plugins.portal.v1.UserIndexResponse;

public record Session(Item user, String sn, UserIndexResponse.Item.Type type) implements Serializable {

}
