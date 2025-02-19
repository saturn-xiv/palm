package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models.Transaction
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.bookkeeper.r.transaction.jpa")
interface TransactionRepository : CrudRepository<Transaction, Int> {
}