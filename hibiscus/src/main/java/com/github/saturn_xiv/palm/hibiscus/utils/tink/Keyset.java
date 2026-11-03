package com.github.saturn_xiv.palm.hibiscus.utils.tink;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.GeneralSecurityException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.google.crypto.tink.InsecureSecretKeyAccess;
import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.TinkProtoKeysetFormat;

public abstract class Keyset {
    protected abstract KeysetHandle generate() throws GeneralSecurityException;

    protected KeysetHandle load(Path file) throws GeneralSecurityException, IOException {
        if (!Files.exists(file)) {
            logger.warn("generate key file {}", file);
            final var keysetHandle = this.generate();
            final byte[] buffer = TinkProtoKeysetFormat.serializeKeyset(keysetHandle, InsecureSecretKeyAccess.get(),
                    RegistryConfiguration.get());
            Files.write(file, buffer);
        }

        logger.debug("load key from {}", file);
        return TinkProtoKeysetFormat.parseKeyset(
                Files.readAllBytes(file),
                InsecureSecretKeyAccess.get(), RegistryConfiguration.get());

    }

    private static final Logger logger = LoggerFactory.getLogger(Keyset.class);
}
