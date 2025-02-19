package com.github.saturn_siv.palm.dahlia.plugins.bbs.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "bbs_forums")
@Entity(name = "bbs.forum")
class Forum(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var slug: String,
    @Column(nullable = false)
    var title: String,
    @Column(nullable = false)
    var description: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status,
    @Column(nullable = false)
    var profile: ByteArray,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
) {
    enum class Status {
        OPENING, LOCKED, DISABLED
    }
}