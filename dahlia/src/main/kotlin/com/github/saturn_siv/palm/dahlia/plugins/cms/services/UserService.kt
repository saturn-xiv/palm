package com.github.saturn_siv.palm.dahlia.plugins.cms.services

import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.UserRepository
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.cms.s.user")
class UserService {
    @Autowired
    private lateinit var userRepository: UserRepository
}