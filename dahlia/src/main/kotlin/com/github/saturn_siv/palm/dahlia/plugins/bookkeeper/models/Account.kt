package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Currency
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_accounts")
@Entity(name = "bookkeeper.account")
class Account(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var name: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var type: Type,
    @Column(nullable = false)
    var balance: Int,
    @Column(nullable = false)
    var memo: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor,
    @Column(nullable = false)
    var profile: ByteArray,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status,
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
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "parent_id")
    var parent: Account?,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "currency_id")
    var currency: Currency,
) {
    enum class Type {
        CASH
    }

    enum class Status {
        OPENING, LOCKED, CLOSED
    }
}