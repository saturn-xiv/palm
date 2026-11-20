package com.github.saturn_xiv.palm.marigold.wechat.requests;

import java.io.Serializable;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;
import java.util.HexFormat;
import java.util.List;

import javax.crypto.BadPaddingException;
import javax.crypto.Cipher;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.NoSuchPaddingException;
import javax.crypto.spec.SecretKeySpec;

import org.apache.commons.codec.digest.MessageDigestAlgorithms;

import com.fasterxml.jackson.annotation.JsonProperty;

public record MessagePushRequest(@JsonProperty("ToUserName") String toUserName, @JsonProperty("Encrypt") String encrypt)
        implements Serializable {

    public String message_body(String appId, String keyStr)
            throws NoSuchAlgorithmException, NoSuchPaddingException, InvalidKeyException, IllegalBlockSizeException,
            BadPaddingException {
        final var key = Base64.getDecoder().decode(keyStr + "=");
        final var buffer = Base64.getDecoder().decode(this.encrypt);

        var secret = new SecretKeySpec(key, "AES");
        var cipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        cipher.init(Cipher.DECRYPT_MODE, secret);

        byte[] tmp = cipher.doFinal(buffer);
        return parse(appId, new String(tmp, StandardCharsets.UTF_8));
    }

    private static String parse(String appId, String message) {
        final int random_len = 16;
        final int msg_len = 4;
        if (message.length() < (random_len + msg_len + appId.length())) {
            throw new IllegalArgumentException();
        }
        if (!message.endsWith(appId)) {
            throw new IllegalArgumentException();
        }
        final var len = Integer.parseInt(message.substring(random_len, random_len + msg_len));
        if (message.length() != random_len + msg_len + len + appId.length()) {
            throw new IllegalArgumentException();
        }
        return message.substring(random_len + msg_len, len);
    }

    public boolean verify(String signature, String token, String timestamp, String nonce)
            throws NoSuchAlgorithmException {
        var items = List.of(token, timestamp, nonce, this.encrypt);
        items.sort(null);
        var str = String.join("", items);
        var md = MessageDigest.getInstance(MessageDigestAlgorithms.SHA_1);
        final var digest = md.digest(str.getBytes(StandardCharsets.UTF_8));
        return signature.equals(HexFormat.of().formatHex(digest));
    }
}
