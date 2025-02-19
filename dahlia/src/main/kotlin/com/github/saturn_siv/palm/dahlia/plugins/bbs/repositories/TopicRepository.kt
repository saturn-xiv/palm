package com.github.saturn_siv.palm.dahlia.plugins.bbs.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bbs.models.Topic
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bbs.r.topic.jpa")
interface TopicRepository : CrudRepository<Topic, Int> {

}