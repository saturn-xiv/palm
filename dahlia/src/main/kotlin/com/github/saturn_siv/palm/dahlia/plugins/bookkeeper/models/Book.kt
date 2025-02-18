package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_books")
@Entity
class Book {
    enum class Status {}
    enum class Template {}

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null


    @Column(nullable = false)
    var uid: String? = null

    @Column(nullable = false)
    var template: String? = null

    @Column(nullable = false)
    var name: String? = null

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
}