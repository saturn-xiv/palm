package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_commodities")
@Entity(name = "bookkeeper.commodity")
class Commodity(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var name: String,
    @Column(nullable = false)
    var amount: Int,
    @Column(nullable = false)
    var unitPrice: Int,
    @Column(nullable = false)
    var totalPrice: Int,
    @Column(nullable = false)
    var memo: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
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
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "vendor_id")
    var vendor: Vendor,
)