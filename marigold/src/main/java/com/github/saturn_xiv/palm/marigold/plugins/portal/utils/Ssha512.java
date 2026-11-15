package com.github.saturn_xiv.palm.marigold.plugins.portal.utils;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;

import org.apache.commons.codec.digest.DigestUtils;
import org.apache.commons.codec.digest.MessageDigestAlgorithms;
import org.springframework.stereotype.Component;

import jakarta.annotation.Resource;

@Component("portal.u.ssha512")
public class Ssha512 {
    // https://mad9scientist.com/dovecot-password-creation-php/
    // https://doc.dovecot.org/2.3/configuration_manual/howto/convert_password_schemes/
    public String compute(String password, int salt_len) throws NoSuchAlgorithmException {
        final var salt = randomUtil.bytes(salt_len);
        return compute(password, salt);
    }

    public String dovecot(String password, int salt_len) throws NoSuchAlgorithmException {
        return String.format("doveadm pw -t '%s' -p %s", compute(password, salt_len), password);
    }

    public boolean verify(String hash, String password) throws NoSuchAlgorithmException {
        if (hash == null || !hash.startsWith(PREFIX)) {
            return false;
        }
        final var buf = Base64.getDecoder().decode(hash.substring(PREFIX.length()));
        final var hash_len = DigestUtils.getSha512Digest().getDigestLength();
        if (buf.length <= hash_len) {
            return false;
        }
        final var salt_len = buf.length - hash_len;
        byte[] salt = new byte[salt_len];
        System.arraycopy(buf, hash_len, salt, 0, salt_len);
        return hash.equals(compute(password, salt));
    }

    private String compute(String password, byte[] salt) throws NoSuchAlgorithmException {
        var digest = MessageDigest.getInstance(MessageDigestAlgorithms.SHA_512);
        digest.update(password.getBytes(StandardCharsets.UTF_8));
        digest.update(salt);
        var buf = digest.digest();

        byte[] tmp = new byte[buf.length + salt.length];
        System.arraycopy(buf, 0, tmp, 0, buf.length);
        System.arraycopy(salt, 0, tmp, buf.length, salt.length);
        return PREFIX + Base64.getEncoder().encode(tmp);
    }

    @Resource
    RandomUtil randomUtil;
    private final static String PREFIX = "{SSHA512}";
}
