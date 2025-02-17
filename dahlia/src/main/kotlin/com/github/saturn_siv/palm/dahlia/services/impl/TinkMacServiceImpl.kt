package com.github.saturn_siv.palm.dahlia.services.impl

import com.github.saturn_siv.palm.dahlia.services.MacService
import com.google.crypto.tink.Mac
import com.google.crypto.tink.RegistryConfiguration
import com.google.crypto.tink.mac.MacConfig
import com.google.crypto.tink.mac.PredefinedMacParameters
import org.springframework.stereotype.Component
import javax.annotation.PostConstruct
import kotlin.io.path.Path


@Component("dahlia.s.mac")
class TinkMacServiceImpl : MacService, TinkKeyset() {
    override fun compute(plain: ByteArray): ByteArray {
        return this.mac.computeMac(plain)
    }

    override fun verify(code: ByteArray, plain: ByteArray) {
        this.mac.verifyMac(code, plain)
    }

    @PostConstruct
    fun init() {
        MacConfig.register()

        val handle = super.load(Path("mac.bin"), PredefinedMacParameters.HMAC_SHA512_512BITTAG)
        this.mac = handle.getPrimitive(RegistryConfiguration.get(), Mac::class.java)
    }

    private lateinit var mac: Mac
}