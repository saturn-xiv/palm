package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Currency
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.cms.r.currency.jpa")
interface CurrencyRepository : CrudRepository<Currency, Int> {
}