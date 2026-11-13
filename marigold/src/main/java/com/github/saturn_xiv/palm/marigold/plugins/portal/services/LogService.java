package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LogRepository;

import jakarta.annotation.Resource;

@Service("portal.s.log")
public class LogService {
    @Resource
    LogRepository logRepository;
}
