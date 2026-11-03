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
import com.google.crypto.tink.mac.MacConfig;
import com.google.crypto.tink.mac.PredefinedMacParameters;

import jakarta.annotation.PostConstruct;

@Component("u.mac")
public class Mac {
    public byte[] compute(byte[] message) throws GeneralSecurityException {
        return this.mac.computeMac(message);
    }

    public void verify(byte[] mac, byte[] message) throws GeneralSecurityException {
        this.mac.verifyMac(mac, message);
    }

    // https://developers.google.com/tink/protect-data-from-tampering#java
    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        MacConfig.register();

        final var file = Path.of("mac.bin");
        if (!Files.exists(file)) {
            logger.warn("generate key file {}", file);
            KeysetHandle handle = KeysetHandle.generateNew(PredefinedMacParameters.HMAC_SHA512_512BITTAG,
                    RegistryConfiguration.get());
            final byte[] keyset = TinkProtoKeysetFormat.serializeKeyset(
                    handle, InsecureSecretKeyAccess.get(), RegistryConfiguration.get());
            Files.write(file, keyset);
        }

        logger.debug("load key from {}", file);
        KeysetHandle handle = TinkProtoKeysetFormat.parseKeyset(
                Files.readAllBytes(file), InsecureSecretKeyAccess.get(), RegistryConfiguration.get());

        this.mac = handle.getPrimitive(RegistryConfiguration.get(), com.google.crypto.tink.Mac.class);

    }

    private com.google.crypto.tink.Mac mac;

    private static final Logger logger = LoggerFactory.getLogger(Mac.class);
}
