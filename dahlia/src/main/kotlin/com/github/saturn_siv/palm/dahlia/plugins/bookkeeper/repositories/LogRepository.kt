package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models.Log
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bookkeeper.r.log.jpa")
interface LogRepository : CrudRepository<Log, Int> {
}