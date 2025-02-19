package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_teams")
@Entity(name = "bookkeeper.team")
class Team(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
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
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "user_id")
    var user: User,
) {
    enum class Status {}
}