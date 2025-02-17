package com.github.saturn_siv.palm.dahlia.services

interface AeadService {
    fun encrypt(plain: ByteArray, associated: ByteArray): ByteArray
    fun decrypt(code: ByteArray, associated: ByteArray): ByteArray
}