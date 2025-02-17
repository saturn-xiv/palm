package com.github.saturn_siv.palm.dahlia.services

import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Component

@Component("dahlia.s.minio")
class MinioService {
    @Value("\${spring.application.name}")
    private lateinit var applicationName: String
}