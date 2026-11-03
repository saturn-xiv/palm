package com.github.saturn_xiv.palm.hibiscus.utils;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.GeneralSecurityException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import com.google.crypto.tink.InsecureSecretKeyAccess;
import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.TinkProtoKeysetFormat;
import com.google.crypto.tink.aead.AeadConfig;
import com.google.crypto.tink.aead.PredefinedAeadParameters;

import jakarta.annotation.PostConstruct;

@Component("u.aead")
public class Aead {
    public byte[] encrypt(byte[] plain, byte[] associated) throws GeneralSecurityException {
        return this.aead.encrypt(plain, associated);
    }

    public byte[] decrypt(byte[] cipher, byte[] associated) throws GeneralSecurityException {
        return this.aead.decrypt(cipher, associated);
    }

    // https://developers.google.com/tink/encrypt-data#java
    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        AeadConfig.register();

        final var file = Path.of("aes.bin");
        if (!Files.exists(file)) {
            logger.warn("generating key file {}", file);
            KeysetHandle handle = KeysetHandle.generateNew(PredefinedAeadParameters.AES256_GCM,
                    RegistryConfiguration.get());
            final byte[] keyset = TinkProtoKeysetFormat.serializeKeyset(
                    handle, InsecureSecretKeyAccess.get(), RegistryConfiguration.get());
            Files.write(file, keyset);
        }

        logger.debug("load key from {}", file);
        KeysetHandle handle = TinkProtoKeysetFormat.parseKeyset(
                Files.readAllBytes(file), InsecureSecretKeyAccess.get(), RegistryConfiguration.get());
        this.aead = handle.getPrimitive(RegistryConfiguration.get(), com.google.crypto.tink.Aead.class);
    }

    private com.google.crypto.tink.Aead aead;

    private static final Logger logger = LoggerFactory.getLogger(Aead.class);
}
