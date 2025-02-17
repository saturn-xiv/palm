package com.github.saturn_siv.palm.dahlia.services.impl

import com.google.crypto.tink.InsecureSecretKeyAccess
import com.google.crypto.tink.KeysetHandle
import com.google.crypto.tink.Parameters
import com.google.crypto.tink.TinkJsonProtoKeysetFormat
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.nio.file.Files
import java.nio.file.Path
import kotlin.io.path.exists
import kotlin.text.Charsets.UTF_8

open class TinkKeyset {
    fun load(file: Path, type: Parameters): KeysetHandle {
        if (!file.exists()) {
            logger.warn("create tink keyset file {}", file)
            val handle = KeysetHandle.generateNew(type)
            val key = TinkJsonProtoKeysetFormat.serializeKeyset(handle, InsecureSecretKeyAccess.get())
            Files.write(file, key.toByteArray(UTF_8));
        }
        val key = String(Files.readAllBytes(file), UTF_8)
        return TinkJsonProtoKeysetFormat.parseKeyset(key, InsecureSecretKeyAccess.get())
    }

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(TinkKeyset::class.java);
    }
}