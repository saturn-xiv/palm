package com.github.saturn_xiv.palm.camellia.helpers;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.GeneralSecurityException;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.google.crypto.tink.InsecureSecretKeyAccess;
import com.google.crypto.tink.KeysetHandle;
import com.google.crypto.tink.Parameters;
import com.google.crypto.tink.TinkProtoKeysetFormat;

public class Tink {
    KeysetHandle loadKeyset(Path file, Parameters parameters) throws IOException, GeneralSecurityException {
        if (!Files.exists(file)) {
            logger.warn("couldn't found {}, will be created", file);
            KeysetHandle handle = KeysetHandle.generateNew(parameters);

            byte[] buf = TinkProtoKeysetFormat.serializeKeyset(handle, InsecureSecretKeyAccess.get());
            Files.write(file, buf);
        }
        KeysetHandle handle = TinkProtoKeysetFormat.parseKeyset(Files.readAllBytes(file),
                InsecureSecretKeyAccess.get());
        return handle;
    }

    private static final Logger logger = LoggerFactory.getLogger(Tink.class);
}
