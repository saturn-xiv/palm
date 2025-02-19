package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models.Team
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bookkeeper.r.team.jpa")
interface TeamRepository : CrudRepository<Team, Int> {
}