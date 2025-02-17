package com.github.saturn_siv.palm.dahlia.services

interface MacService {
    fun compute(plain: ByteArray): ByteArray
    fun verify(code: ByteArray, plain: ByteArray)
}