package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "sessions")
@Entity(name = "session")
class Session(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var uid: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var providerType: Type,
    @Column(nullable = false)
    var providerId: Int,
    @Column(nullable = false)
    var ip: String,
    @Column(nullable = false)
    var expiresAt: Instant,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    var user: User,
) {
    enum class Type {
        EMAIL, GOOGLE_OAUTH2, WECHAT_OAUTH2, WECHAT_MINI_PROGRAM
    }
}