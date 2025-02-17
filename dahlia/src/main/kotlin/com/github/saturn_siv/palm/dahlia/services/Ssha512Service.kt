package com.github.saturn_siv.palm.dahlia.services

import org.springframework.stereotype.Component
import java.security.MessageDigest
import java.util.*
import kotlin.random.Random
import kotlin.text.Charsets.UTF_8


@Component("dahlia.s.ssha512")
class Ssha512Service {
    //    https://mad9scientist.com/dovecot-password-creation-php/
    fun compute(plain: String): String {
        val salt = ByteArray(16)
        Random.nextBytes(salt)
        return this.compute(plain, salt)
    }

    fun verify(code: String, plain: String): Boolean {
        if (!code.startsWith(prefix)) {
            return false
        }
        val buf = Base64.getDecoder().decode(code.removePrefix(prefix))
        val salt = buf.slice(64..<buf.size)
        return this.compute(plain, salt.toByteArray()) == code

    }


    private fun compute(plain: String, salt: ByteArray): String {
        val md = MessageDigest.getInstance("SHA-512")
        val digest = md.digest(plain.toByteArray(UTF_8) + salt)
//        val base64=Base64.Default.withPadding(Base64.PaddingOption.ABSENT)
        return "${this.prefix}${Base64.getEncoder().withoutPadding().encodeToString(digest + salt)}"
    }

    private val prefix: String = "{SSHA512}"
}