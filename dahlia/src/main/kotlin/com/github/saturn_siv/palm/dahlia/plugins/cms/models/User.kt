package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "users")
@Entity
class User {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var uid: String? = null

    @Column(nullable = false)
    var lang: String? = null

    @Column(nullable = false)
    var timezone: String? = null

    @Column(nullable = false)
    var signInCount: Int? = null

    @Column
    var currentSignInAt: Instant? = null

    @Column
    var currentSignInIp: String? = null

    @Column
    var lastSignInAt: Instant? = null

    @Column
    var lastSignInIp: String? = null

    @Column
    var lockedAt: Instant? = null

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    val attachments = mutableListOf<Attachment>()
}