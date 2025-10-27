package com.github.saturn_xiv.palm.camellia.helpers;

import jakarta.annotation.PostConstruct;

import java.io.IOException;
import java.nio.file.Paths;
import java.security.GeneralSecurityException;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;

import org.springframework.stereotype.Component;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.jwt.JwtHmacParameters;
import com.google.crypto.tink.jwt.JwtMac;
import com.google.crypto.tink.jwt.JwtMacConfig;
import com.google.crypto.tink.jwt.JwtValidator;
import com.google.crypto.tink.jwt.RawJwt;
import com.google.crypto.tink.jwt.VerifiedJwt;
import com.google.crypto.tink.jwt.JwtEcdsaParameters.KidStrategy;

@Component("palm.camellia.jwt-helper")
public class JwtHelper extends Tink {

    public String sign(String issuer, String subject, List<String> audiences, long ttl,
            Map<String, String> extra)
            throws GeneralSecurityException {
        final var now = Instant.now();
        var builder = RawJwt.newBuilder()
                .setJwtId(UUID.randomUUID().toString())
                .setIssuer(issuer)
                .setSubject(subject)
                .setAudiences(audiences)
                .setNotBefore(now.minusSeconds(1))
                .setIssuedAt(now)
                .setExpiration(Instant.now().plusSeconds(ttl));
        for (var it : extra.entrySet()) {
            builder = builder.addJsonObjectClaim(it.getKey(), it.getValue());
        }
        RawJwt raw = builder
                .build();
        return this.jwt.computeMacAndEncode(raw);
    }

    public String sign(String issuer, String subject, List<String> audiences)
            throws GeneralSecurityException {
        return this.sign(issuer, subject, audiences, 60 * 60 * 24, new HashMap<>());
    }

    public String sign(String issuer, String subject)
            throws GeneralSecurityException {
        return this.sign(issuer, subject, new ArrayList<>(), 60 * 60 * 24, new HashMap<>());
    }

    public String sign(String issuer, String subject, long ttl)
            throws GeneralSecurityException {
        return this.sign(issuer, subject, new ArrayList<>(), ttl, new HashMap<>());
    }

    public VerifiedJwt verify(String token, String issuer, String audience) throws GeneralSecurityException {
        JwtValidator validator = JwtValidator.newBuilder().expectIssuer(issuer).expectAudience(audience).build();
        return this.jwt.verifyMacAndDecode(token, validator);
    }

    public VerifiedJwt verify(String token, String issuer) throws GeneralSecurityException {
        JwtValidator validator = JwtValidator.newBuilder().expectIssuer(issuer).build();
        return this.jwt.verifyMacAndDecode(token, validator);
    }

    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        JwtMacConfig.register();
        var handle = super.loadKeyset(Paths.get("jwt.bin"),
                JwtHmacParameters.builder().setAlgorithm(JwtHmacParameters.Algorithm.HS512).setKeySizeBytes(128)
                        .setKidStrategy(JwtHmacParameters.KidStrategy.IGNORED)
                        .build());
        this.jwt = handle.getPrimitive(RegistryConfiguration.get(), JwtMac.class);
    }

    private JwtMac jwt;
}
