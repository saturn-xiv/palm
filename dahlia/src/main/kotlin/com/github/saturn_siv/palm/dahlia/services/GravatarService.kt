package com.github.saturn_siv.palm.dahlia.services

import org.springframework.stereotype.Component
import java.security.MessageDigest
import kotlin.text.Charsets.UTF_8

@Component("dahlia.s.gravatar")
class GravatarService {
    //    TODO
//    https://docs.gravatar.com/api/avatars/hash/
    @OptIn(ExperimentalStdlibApi::class)
    fun url(email: String): String {
        val md = MessageDigest.getInstance("SHA-256")
        val digest = md.digest(email.lowercase().trim().toByteArray(UTF_8))
        return "https://gravatar.com/avatar/${digest.toHexString()}"
    }
}