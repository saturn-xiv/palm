package com.github.saturn_siv.palm.dahlia.plugins.bbs.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "bbs_forums")
@Entity
class Forum {
    enum class Status {
        OPENING, LOCKED, DISABLED
    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var slug: String? = null

    @Column(nullable = false)
    var title: String? = null

    @Column(nullable = false)
    var description: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status? = null

    @Column(nullable = false)
    var profile: ByteArray? = null

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null


}