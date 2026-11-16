package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.AttachmentRepository;

import jakarta.annotation.Resource;

@Controller("portal.c.attachments")
@RequestMapping("/attachments")
public class AttachmentsController {

    @GetMapping("/{sn}")
    public void show(@PathVariable String sn) {
        final var it = attachmentRepository.findBySn(sn).get();
        // TODO
        if (it.isPublic_()) {

        }
    }

    @PostMapping
    public void upload() {
        // TODO
    }

    @Resource
    AttachmentRepository attachmentRepository;
}
