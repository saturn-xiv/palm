package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_commodities")
@Entity
class Commodity {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var name: String? = null

    @Column(nullable = false)
    var amount: Int? = null

    @Column(nullable = false)
    var unitPrice: Int? = null

    @Column(nullable = false)
    var totalPrice: Int? = null

    @Column(nullable = false)
    var memo: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

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

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "vendor_id")
    val vendor: Vendor? = null
}