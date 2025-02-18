package com.github.saturn_siv.palm.dahlia.plugins.cms.services

import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.AttachmentRepository
import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.AttachmentResourceRepository
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.cms.s.attachment")
class AttachmentService {
    @Autowired
    private lateinit var attachmentRepository: AttachmentRepository

    @Autowired
    private lateinit var attachmentResourceRepository: AttachmentResourceRepository

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(AttachmentService::class.java);
    }
}