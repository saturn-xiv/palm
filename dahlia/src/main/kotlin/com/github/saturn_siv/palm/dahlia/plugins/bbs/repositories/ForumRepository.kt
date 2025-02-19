package com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bbs.models.Forum
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bbs.r.forum.jpa")
interface ForumRepository : CrudRepository<Forum, Int> {
}