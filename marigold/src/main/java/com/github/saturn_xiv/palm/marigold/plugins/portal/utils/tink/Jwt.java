package com.github.saturn_xiv.palm.marigold.plugins.portal.utils.tink;

import java.io.IOException;
import java.io.Serializable;
import java.nio.file.Path;
import java.security.GeneralSecurityException;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.List;
import java.util.UUID;

import org.apache.commons.lang3.tuple.ImmutablePair;
import org.springframework.stereotype.Component;

import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.jwt.JwtHmacParameters;
import com.google.crypto.tink.jwt.JwtMac;
import com.google.crypto.tink.jwt.JwtMacConfig;
import com.google.crypto.tink.jwt.JwtValidator;
import com.google.crypto.tink.jwt.RawJwt;
import com.google.crypto.tink.jwt.VerifiedJwt;
import com.google.gson.Gson;

import jakarta.annotation.PostConstruct;

@Component("portal.u.jwt.tink")
public class Jwt extends Keyset {
        public ImmutablePair<String, String> verify(String token, String issuer, String audience)
                        throws GeneralSecurityException {
                JwtValidator validator = JwtValidator.newBuilder()
                                .expectIssuer(issuer)
                                .expectAudience(audience)
                                .expectIssuedInThePast()
                                .build();

                VerifiedJwt raw = this.jwt.verifyMacAndDecode(token, validator);
                String subject = raw.getSubject();
                String payload = raw.getStringClaim(PAYLOAD);
                return ImmutablePair.of(subject, payload);
        }

        public <V extends Serializable> ImmutablePair<String, V> verify(String token, String issuer, String audience,
                        Class<V> clazz)
                        throws GeneralSecurityException {
                JwtValidator validator = JwtValidator.newBuilder()
                                .expectIssuer(issuer)
                                .expectAudience(audience)
                                .expectIssuedInThePast()
                                .build();

                VerifiedJwt raw = this.jwt.verifyMacAndDecode(token, validator);
                String subject = raw.getSubject();
                V payload = this.gson.fromJson(raw.getJsonObjectClaim(PAYLOAD), clazz);
                return ImmutablePair.of(subject, payload);
        }

        public <S> String sign(String issuer, String subject, List<String> audiences, S payload,
                        long ttl)
                        throws GeneralSecurityException {

                final var now = Instant.now();
                RawJwt raw = RawJwt.newBuilder()
                                .setJwtId(UUID.randomUUID().toString())
                                .setIssuer(issuer)
                                .setSubject(subject)
                                .setAudiences(audiences)
                                .setIssuedAt(now)
                                .setNotBefore(now)
                                .setExpiration(now.plus(ttl, ChronoUnit.SECONDS))
                                .addJsonObjectClaim(PAYLOAD, gson.toJson(payload))
                                .build();

                return this.jwt.computeMacAndEncode(raw);
        }

        public String sign(String issuer, String subject, List<String> audiences, String payload,
                        long ttl)
                        throws GeneralSecurityException {

                final var now = Instant.now();
                RawJwt raw = RawJwt.newBuilder()
                                .setIssuer(issuer)
                                .setSubject(subject)
                                .setAudiences(audiences)
                                .setIssuedAt(now)
                                .setNotBefore(now)
                                .setExpiration(now.plus(ttl, ChronoUnit.SECONDS))
                                .addStringClaim(PAYLOAD, payload)
                                .build();

                return this.jwt.computeMacAndEncode(raw);
        }

        @Override
        protected KeysetHandle generate() throws GeneralSecurityException {
                return KeysetHandle.generateNew(JwtHmacParameters.builder()
                                .setKeySizeBytes(64)
                                .setAlgorithm(JwtHmacParameters.Algorithm.HS512)
                                .setKidStrategy(JwtHmacParameters.KidStrategy.BASE64_ENCODED_KEY_ID)
                                .build(),
                                RegistryConfiguration.get());

        }

        // https://developers.google.com/tink/create-jwts-signature#java
        @PostConstruct
        void init() throws GeneralSecurityException, IOException {
                this.gson = new Gson();
                JwtMacConfig.register();

                final var keysetHandle = super.load(Path.of("jwt.bin"));
                this.jwt = keysetHandle.getPrimitive(RegistryConfiguration.get(), JwtMac.class);
        }

        private JwtMac jwt;
        private Gson gson;
        private final static String PAYLOAD = "ext";
}
