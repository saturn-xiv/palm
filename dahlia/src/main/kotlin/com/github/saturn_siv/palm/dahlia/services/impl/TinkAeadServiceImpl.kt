package com.github.saturn_siv.palm.dahlia.services.impl

import com.github.saturn_siv.palm.dahlia.services.AeadService
import com.google.crypto.tink.Aead
import com.google.crypto.tink.RegistryConfiguration
import com.google.crypto.tink.aead.AeadConfig
import com.google.crypto.tink.aead.PredefinedAeadParameters
import org.springframework.stereotype.Component
import javax.annotation.PostConstruct
import kotlin.io.path.Path

@Component("dahlia.s.aead-by-tink")
class TinkAeadServiceImpl : AeadService, TinkKeyset() {
    override fun encrypt(plain: ByteArray, associated: ByteArray): ByteArray {
        return this.aead.encrypt(plain, associated)
    }

    override fun decrypt(code: ByteArray, associated: ByteArray): ByteArray {
        return this.aead.decrypt(code, associated)
    }

    @PostConstruct
    fun init() {
        AeadConfig.register()
        val handle = super.load(Path("aead.bin"), PredefinedAeadParameters.AES256_GCM)
        this.aead = handle.getPrimitive(RegistryConfiguration.get(), Aead::class.java);
    }


    private lateinit var aead: Aead
}