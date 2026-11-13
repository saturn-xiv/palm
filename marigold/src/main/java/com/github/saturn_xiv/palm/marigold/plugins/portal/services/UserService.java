package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LogRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserRepository;

import jakarta.annotation.Resource;

@Service("portal.s.user")
public class UserService {
    @Resource
    LogRepository logRepository;
    @Resource
    UserRepository userRepository;
}
