package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;

import org.apache.commons.codec.digest.MessageDigestAlgorithms;
import org.springframework.stereotype.Component;

@Component("portal.u.gravatar")
public class Gravatar {
    // https://docs.gravatar.com/rest/hash/
    public String hash(String rawEmail) throws NoSuchAlgorithmException {
        final var email = rawEmail.trim().toLowerCase();
        var digest = MessageDigest.getInstance(MessageDigestAlgorithms.SHA_256);
        var buf = digest.digest(email.getBytes(StandardCharsets.UTF_8));
        return HexFormat.of().formatHex(buf);
    }

    // https://docs.gravatar.com/sdk/images/
    public String image(String email) throws NoSuchAlgorithmException {
        return String.format("https://gravatar.com/avatar/%s", hash(email));
    }
}
