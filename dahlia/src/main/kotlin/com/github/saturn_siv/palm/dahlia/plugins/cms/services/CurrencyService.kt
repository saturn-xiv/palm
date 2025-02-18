package com.github.saturn_siv.palm.dahlia.plugins.cms.services

import com.github.saturn_siv.palm.dahlia.plugins.cms.repositories.CurrencyRepository
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.cms.s.currency")
class CurrencyService {
    @Autowired
    private lateinit var currencyRepository: CurrencyRepository

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(CurrencyService::class.java);
    }
}