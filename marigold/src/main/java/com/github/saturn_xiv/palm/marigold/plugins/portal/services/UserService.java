package com.github.saturn_xiv.palm.marigold.plugins.portal.services;

import org.springframework.stereotype.Service;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.EmailUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.GoogleOauth2UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LogRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.SmsUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserBanRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserContactRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatMiniProgramUserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.WechatOauth2UserRepository;

import jakarta.annotation.Resource;

@Service("portal.s.user")
public class UserService {
    @Resource
    LogRepository logRepository;
    @Resource
    UserRepository userRepository;
    @Resource
    EmailUserRepository emailUserRepository;
    @Resource
    GoogleOauth2UserRepository googleOauth2UserRepository;
    @Resource
    SmsUserRepository smsUserRepository;
    @Resource
    WechatMiniProgramUserRepository wechatMiniProgramUserRepository;
    @Resource
    WechatOauth2UserRepository wechatOauth2UserRepository;
    @Resource
    UserBanRepository userBanRepository;
    @Resource
    UserContactRepository userContactRepository;
}
