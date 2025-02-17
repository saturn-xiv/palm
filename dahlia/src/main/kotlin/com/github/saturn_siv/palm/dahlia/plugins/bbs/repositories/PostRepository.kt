package com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bbs.models.Post
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.bbs.r.post")
interface PostRepository : CrudRepository<Post, Int>, JpaRepository<Post, Int> {
}