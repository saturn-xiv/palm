package com.github.saturn_siv.palm.dahlia.plugins.bbs.services

import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.ForumRepository
import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.PostRepository
import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.TopicRepository
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.bbs.s.bbs")
class BbsService {
    @Autowired
    private lateinit var forumRepository: ForumRepository

    @Autowired
    private lateinit var postRepository: PostRepository

    @Autowired
    private lateinit var topicRepository: TopicRepository

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(BbsService::class.java);
    }
}