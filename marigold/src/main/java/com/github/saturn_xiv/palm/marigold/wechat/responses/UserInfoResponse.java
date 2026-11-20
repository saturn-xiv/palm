package com.github.saturn_xiv.palm.marigold.wechat.responses;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;

public class UserInfoResponse extends ErrorResponse {

    private String nickname;
    @JsonProperty("openid")
    private String openid;
    @JsonProperty("headimgurl")
    private String headImgUrl;
    private List<String> privilege;
    @JsonProperty("unionid")
    private String unionId;

    public String getNickname() {
        return nickname;
    }

    public void setNickname(String nickname) {
        this.nickname = nickname;
    }

    public String getOpenid() {
        return openid;
    }

    public void setOpenid(String openid) {
        this.openid = openid;
    }

    public String getHeadImgUrl() {
        return headImgUrl;
    }

    public void setHeadImgUrl(String headImgUrl) {
        this.headImgUrl = headImgUrl;
    }

    public List<String> getPrivilege() {
        return privilege;
    }

    public void setPrivilege(List<String> privilege) {
        this.privilege = privilege;
    }

    public String getUnionId() {
        return unionId;
    }

    public void setUnionId(String unionId) {
        this.unionId = unionId;
    }

}
