package com.github.saturn_xiv.palm.marigold.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.repositories.LogRepository;

import jakarta.annotation.Resource;

@Service("s.log")
public class LogService {
    @Resource
    LogRepository logRepository;
}
