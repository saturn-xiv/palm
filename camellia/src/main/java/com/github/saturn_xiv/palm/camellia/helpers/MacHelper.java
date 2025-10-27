package com.github.saturn_xiv.palm.camellia.helpers;

import java.io.IOException;
import java.nio.file.Paths;
import java.security.GeneralSecurityException;

import jakarta.annotation.PostConstruct;

import org.springframework.stereotype.Component;
import com.google.crypto.tink.Mac;
import com.google.crypto.tink.RegistryConfiguration;
import com.google.crypto.tink.mac.MacConfig;
import com.google.crypto.tink.mac.PredefinedMacParameters;

@Component("palm.camellia.mac-helper")
public class MacHelper extends Tink {

    public byte[] compute(byte[] data) throws GeneralSecurityException {
        return this.mac.computeMac(data);
    }

    public void verify(byte[] mac, byte[] data) throws GeneralSecurityException {
        this.mac.verifyMac(mac, data);
    }

    @PostConstruct
    void init() throws IOException, GeneralSecurityException {
        MacConfig.register();
        var handle = super.loadKeyset(Paths.get("mac.bin"), PredefinedMacParameters.HMAC_SHA512_512BITTAG);
        this.mac = handle.getPrimitive(RegistryConfiguration.get(), Mac.class);
    }

    private Mac mac;

}
