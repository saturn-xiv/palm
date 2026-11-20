package com.github.saturn_xiv.palm.marigold.wechat.requests;

import java.io.Serializable;

import com.fasterxml.jackson.annotation.JsonProperty;

public record AuthorizationChangeRequest(@JsonProperty("ToUserName") String toUserName,
        @JsonProperty("FromUserName") String fromUserName, @JsonProperty("CreateTime") long createTime,
        @JsonProperty("MsgType") String type, @JsonProperty("Event") String event,
        @JsonProperty("OpenID") String openId, @JsonProperty("AppID") String appId,
        @JsonProperty("RevokeInfo") String revokeInfo) implements Serializable {
    public boolean isRevoke() {
        return "301".equals(this.revokeInfo);
    }
}
