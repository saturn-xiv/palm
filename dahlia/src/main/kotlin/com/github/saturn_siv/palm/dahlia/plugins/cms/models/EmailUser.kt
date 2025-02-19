package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "email_users")
@Entity(name = "user.by-email")
class EmailUser(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var realName: String,
    @Column(nullable = false)
    var email: String,
    @Column(nullable = false)
    var password: ByteArray,
    @Column(nullable = false)
    var salt: ByteArray,
    @Column(nullable = false)
    var avatar: String,
    @Column
    var confirmedAt: Instant?,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    var user: User,
)