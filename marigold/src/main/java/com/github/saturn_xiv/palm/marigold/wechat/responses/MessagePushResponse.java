package com.github.saturn_xiv.palm.marigold.wechat.responses;

import java.io.Serializable;

import com.fasterxml.jackson.annotation.JsonProperty;

public class MessagePushResponse
        implements Serializable {
    public MessagePushResponse() {
    }

    public <T> MessagePushResponse(String appId, String keyStr, T message) {

    }

    @JsonProperty("Encrypt")
    private String encrypt;
    @JsonProperty("MsgSignature")
    private String signature;
    @JsonProperty("TimeStamp")
    private long timeStamp;
    @JsonProperty("Nonce")
    private long nonce;

    public String getEncrypt() {
        return encrypt;
    }

    public void setEncrypt(String encrypt) {
        this.encrypt = encrypt;
    }

    public String getSignature() {
        return signature;
    }

    public void setSignature(String signature) {
        this.signature = signature;
    }

    public long getTimeStamp() {
        return timeStamp;
    }

    public void setTimeStamp(long timeStamp) {
        this.timeStamp = timeStamp;
    }

    public long getNonce() {
        return nonce;
    }

    public void setNonce(long nonce) {
        this.nonce = nonce;
    }

}
