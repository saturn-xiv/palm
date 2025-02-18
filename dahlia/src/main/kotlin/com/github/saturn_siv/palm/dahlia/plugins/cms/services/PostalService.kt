package com.github.saturn_siv.palm.dahlia.plugins.cms.services

import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.PostalAddressRepository
import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.PostalRecipientRepository
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.cms.s.postal")
class PostalService {
    @Autowired
    private lateinit var addressRepository: PostalAddressRepository

    @Autowired
    private lateinit var recipientRepository: PostalRecipientRepository

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(PostalService::class.java);
    }
}