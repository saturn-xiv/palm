package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "vote_logs")
@Entity(name = "vote-log")
class VoteLog(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var resourceType: String,
    @Column(nullable = false)
    var resourceId: Int,
    @Column(nullable = false)
    var username: String,
    @Column(nullable = false)
    var ip: String,
    @Column(nullable = false)
    var memo: String,
    @Column(nullable = false)
    var star: Int,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "user_id")
    var user: User?
)