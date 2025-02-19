package com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bbs.models.Post
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bbs.r.post.jpa")
interface PostRepository : CrudRepository<Post, Int> {
}