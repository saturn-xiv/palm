package com.github.saturn_siv.palm.dahlia.services.impl

import com.github.saturn_siv.palm.dahlia.services.JwtService
import com.google.crypto.tink.RegistryConfiguration
import com.google.crypto.tink.jwt.JwtMac
import com.google.crypto.tink.jwt.JwtSignatureConfig
import com.google.crypto.tink.jwt.JwtValidator
import com.google.crypto.tink.jwt.RawJwt
import com.google.crypto.tink.mac.PredefinedMacParameters
import org.springframework.stereotype.Component
import java.time.Instant
import javax.annotation.PostConstruct
import kotlin.io.path.Path

@Component("dahlia.s.jwt-by-tink")
class TinkJwtServiceImpl : JwtService, TinkKeyset() {
    override fun sign(
        issuer: String,
        subject: String,
        audiences: Set<String>,
        notBefore: Instant,
        expiresAt: Instant
    ): String {
        val rawJwt =
            RawJwt.newBuilder().setAudiences(audiences.toList()).setSubject(subject).setIssuer(issuer)
                .setIssuedAt(Instant.now())
                .setNotBefore(notBefore).setExpiration(expiresAt).build()
        return this.jwt.computeMacAndEncode(rawJwt)
    }

    override fun verify(token: String, issuer: String, audience: String): String {
        val validator = JwtValidator.newBuilder().expectIssuer(issuer).expectAudience(audience).build()
        val verifiedJwt = this.jwt.verifyMacAndDecode(token, validator)
        return verifiedJwt.subject
    }

    @PostConstruct
    fun init() {
        JwtSignatureConfig.register()
        val handle = super.load(Path("jwt.bin"), PredefinedMacParameters.HMAC_SHA512_512BITTAG)
        this.jwt = handle.getPrimitive(RegistryConfiguration.get(), JwtMac::class.java)
    }


    private lateinit var jwt: JwtMac
}