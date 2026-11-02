package com.github.saturn_xiv.palm.hibiscus.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.hibiscus.repositories.LogRepository;

import jakarta.annotation.Resource;

@Service("s.log")
public class LogService {
    @Resource
    LogRepository logRepository;
}
