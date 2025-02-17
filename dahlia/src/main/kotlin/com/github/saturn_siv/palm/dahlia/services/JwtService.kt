package com.github.saturn_siv.palm.dahlia.services

import java.time.Instant

interface JwtService {
    fun sign(issuer: String, subject: String, audiences: Set<String>, notBefore: Instant, expiresAt: Instant): String
    fun verify(token: String, issuer: String, audience: String): String
}