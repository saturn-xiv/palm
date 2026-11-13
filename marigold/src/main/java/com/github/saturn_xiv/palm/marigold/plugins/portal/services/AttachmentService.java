package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.AttachmentRepository;

import jakarta.annotation.Resource;

@Service("portal.s.attachment")
public class AttachmentService {
    @Resource
    AttachmentRepository attachmentRepository;
}
