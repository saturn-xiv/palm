package com.github.saturn_xiv.palm.marigold.plugins.portal.utils.tink;

import java.io.IOException;
import java.nio.file.Path;
import java.security.GeneralSecurityException;

import org.springframework.stereotype.Component;

import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.aead.AeadConfig;
import com.google.crypto.tink.aead.PredefinedAeadParameters;

import jakarta.annotation.PostConstruct;

@Component("portal.u.aead.tink")
public class Aead extends Keyset {
    public byte[] encrypt(byte[] plain, byte[] associated) throws GeneralSecurityException {
        return this.aead.encrypt(plain, associated);
    }

    public byte[] decrypt(byte[] cipher, byte[] associated) throws GeneralSecurityException {
        return this.aead.decrypt(cipher, associated);
    }

    @Override
    protected KeysetHandle generate() throws GeneralSecurityException {
        return KeysetHandle.generateNew(PredefinedAeadParameters.AES256_GCM,
                RegistryConfiguration.get());
    }

    // https://developers.google.com/tink/encrypt-data#java
    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        AeadConfig.register();

        final var handle = super.load(Path.of("aes.bin"));
        this.aead = handle.getPrimitive(RegistryConfiguration.get(), com.google.crypto.tink.Aead.class);
    }

    private com.google.crypto.tink.Aead aead;

}
