package com.github.saturn_xiv.palm.camellia.helpers;

import java.io.IOException;
import java.nio.file.Paths;
import java.security.GeneralSecurityException;

import jakarta.annotation.PostConstruct;

import org.springframework.stereotype.Component;
import com.google.crypto.tink.Aead;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.aead.AeadConfig;
import com.google.crypto.tink.aead.PredefinedAeadParameters;

@Component("palm.camellia.aead-helper")
public class AeadHelper extends Tink {

    public byte[] encrypt(byte[] plain, byte[] associated) throws GeneralSecurityException {
        return this.aead.encrypt(plain, associated);
    }

    public byte[] decrypt(byte[] cipher, byte[] associated) throws GeneralSecurityException {
        return this.aead.decrypt(cipher, associated);
    }

    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        AeadConfig.register();
        var handle = super.loadKeyset(Paths.get("aead.bin"), PredefinedAeadParameters.AES256_GCM);
        this.aead = handle.getPrimitive(RegistryConfiguration.get(), Aead.class);
    }

    private Aead aead;

}
