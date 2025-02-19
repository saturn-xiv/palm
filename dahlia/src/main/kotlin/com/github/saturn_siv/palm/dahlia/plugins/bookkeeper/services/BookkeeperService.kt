package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.services

import com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.repositories.*
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service

@Service("dahlia.bookkeeper.s.bookkeeper")
class BookkeeperService {
    @Autowired
    private lateinit var accountRepository: AccountRepository

    @Autowired
    private lateinit var bookRepository: BookRepository

    @Autowired
    private lateinit var commodityRepository: CommodityRepository

    @Autowired
    private lateinit var orderRepository: OrderRepository

    @Autowired
    private lateinit var teamRepository: TeamRepository

    @Autowired
    private lateinit var transactionRepository: TransactionRepository

    @Autowired
    private lateinit var vendorRepository: VendorRepository

    @Autowired
    private lateinit var logRepository: LogRepository

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(BookkeeperService::class.java);
    }
}