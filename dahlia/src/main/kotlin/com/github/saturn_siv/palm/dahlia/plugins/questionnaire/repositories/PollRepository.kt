package com.github.saturn_siv.palm.dahlia.plugins.questionnaire.repositories

import com.github.saturn_siv.palm.dahlia.plugins.questionnaire.models.Poll
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.questionnaire.r.poll.jpa")
interface PollRepository : CrudRepository<Poll, Int> {
}