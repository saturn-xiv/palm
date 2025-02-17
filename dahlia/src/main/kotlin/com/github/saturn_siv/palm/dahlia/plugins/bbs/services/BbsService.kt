package com.github.saturn_siv.palm.dahlia.plugins.bbs.services

import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.ForumRepository
import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.PostRepository
import com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories.TopicRepository
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.bbs.s")
class BbsService {
    @Autowired
    private lateinit var forumRepository: ForumRepository

    @Autowired
    private lateinit var postRepository: PostRepository

    @Autowired
    private lateinit var topicRepository: TopicRepository
}