package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Currency
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_orders")
@Entity
class Order {
    enum class Status {}

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var sn: String? = null

    @Column(nullable = false)
    var price: Int? = null

    @Column(nullable = false)
    var memo: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor? = null

    @Column(nullable = false)
    var profile: ByteArray? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status? = null


    @Column
    var placedAt: Instant? = null

    @Column
    var paidAt: Instant? = null

    @Column
    var confirmedAt: Instant? = null

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

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "placed_by")
    val placedBy: User? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "confirmed_by")
    val confirmedBy: User? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "paid_by")
    val paidBy: User? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "currency_id")
    val currency: Currency? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "debtor_id")
    val debtor: Account? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "creditor_id")
    val creditor: Account? = null
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "vendor_id")
    val vendor: Vendor? = null
}