package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_transactions")
@Entity
class Transaction {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var content: ByteArray? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "team_id")
    val team: Team? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "book_id")
    val book: Book? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "order_id")
    val order: Order? = null
}