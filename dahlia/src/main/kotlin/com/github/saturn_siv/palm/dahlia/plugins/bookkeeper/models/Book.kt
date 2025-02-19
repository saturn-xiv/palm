package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_books")
@Entity(name = "bookkeeper.book")
class Book(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var uid: String,
    @Column(nullable = false)
    var template: String,
    @Column(nullable = false)
    var name: String,
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
) {
    enum class Status {}
    enum class Template {}


}