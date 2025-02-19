package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_transactions")
@Entity(name = "bookkeeper,transaction")
class Transaction(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var content: ByteArray,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "team_id")
    var team: Team,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "book_id")
    var book: Book,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "order_id")
    var order: Order,
)