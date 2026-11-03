package com.github.saturn_xiv.palm.hibiscus.utils.tink;

import java.io.IOException;
import java.nio.file.Path;
import java.security.GeneralSecurityException;

import org.springframework.stereotype.Component;

import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.mac.MacConfig;
import com.google.crypto.tink.mac.PredefinedMacParameters;

import jakarta.annotation.PostConstruct;

@Component("u.mac.tink")
public class Mac extends Keyset {
    public byte[] compute(byte[] message) throws GeneralSecurityException {
        return this.mac.computeMac(message);
    }

    public void verify(byte[] mac, byte[] message) throws GeneralSecurityException {
        this.mac.verifyMac(mac, message);
    }

    @Override
    protected KeysetHandle generate() throws GeneralSecurityException {
        return KeysetHandle.generateNew(PredefinedMacParameters.HMAC_SHA512_512BITTAG,
                RegistryConfiguration.get());
    }

    // https://developers.google.com/tink/protect-data-from-tampering#java
    @PostConstruct
    void init() throws GeneralSecurityException, IOException {
        MacConfig.register();

        final var keysetHandle = super.load(Path.of("mac.bin"));
        this.mac = keysetHandle.getPrimitive(RegistryConfiguration.get(), com.google.crypto.tink.Mac.class);

    }

    private com.google.crypto.tink.Mac mac;

}
