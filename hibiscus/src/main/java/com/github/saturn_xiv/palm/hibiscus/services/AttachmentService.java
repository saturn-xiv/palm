package com.github.saturn_xiv.palm.hibiscus.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.hibiscus.repositories.AttachmentRepository;

import jakarta.annotation.Resource;

@Service("s.attachment")
public class AttachmentService {
    @Resource
    AttachmentRepository attachmentRepository;
}
