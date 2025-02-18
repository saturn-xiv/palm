package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Currency
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_accounts")
@Entity
class Account {
    enum class Type {
        CASH
    }

    enum class Status {}

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var name: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var type: Type? = null

    @Column(nullable = false)
    var balance: Int? = null

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
    @JoinColumn(name = "parent_id")
    val parent: Account? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "currency_id")
    val currency: Currency? = null
}