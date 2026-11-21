package com.github.saturn_xiv.palm.marigold.wechat.responses;

import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;

import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

import org.apache.commons.codec.digest.HmacAlgorithms;

import com.fasterxml.jackson.annotation.JsonProperty;

public class JsCode2SessionResponse extends ErrorResponse {
    public String signature() throws NoSuchAlgorithmException, InvalidKeyException {
        final var algorithm = HmacAlgorithms.HMAC_SHA_256.name();
        var mac = Mac.getInstance(algorithm);
        var secretKeySpec = new SecretKeySpec(
                this.sessionKey.getBytes(StandardCharsets.UTF_8),
                algorithm);
        mac.init(secretKeySpec);
        final var buf = mac.doFinal("".getBytes(StandardCharsets.UTF_8));
        return HexFormat.of().formatHex(buf);
    }

    @JsonProperty("openid")
    private String openId;
    @JsonProperty("session_key")
    private String sessionKey;
    @JsonProperty("unionid")
    private String unionId;

    public String getOpenId() {
        return openId;
    }

    public void setOpenId(String openId) {
        this.openId = openId;
    }

    public String getSessionKey() {
        return sessionKey;
    }

    public void setSessionKey(String sessionKey) {
        this.sessionKey = sessionKey;
    }

    public String getUnionId() {
        return unionId;
    }

    public void setUnionId(String unionId) {
        this.unionId = unionId;
    }

}
