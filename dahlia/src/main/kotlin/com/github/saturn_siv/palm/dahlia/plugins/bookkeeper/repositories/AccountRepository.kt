package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models.Account
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bookkeeper.r.account.jpa")
interface AccountRepository : CrudRepository<Account, Int> {
}