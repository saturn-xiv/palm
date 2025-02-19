package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "google_oauth2_users")
@Entity(name = "user.by-google.oauth2")
class GoogleOauth2User(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var subject: String,
    @Column
    var email: String,
    @Column(nullable = false)
    var emailVerified: Boolean,
    @Column
    var name: String?,
    @Column
    var picture: String?,
    @Column
    var locale: String?,
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