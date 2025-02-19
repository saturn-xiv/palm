package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Log
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.log.jpa")
interface LogRepository : CrudRepository<Log, Int> {
}