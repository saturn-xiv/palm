package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "trash_bin")
@Entity(name = "trash-bin")
class TrashBin(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var resourceType: String,
    @Column(nullable = false)
    var resourceId: Int,
    @Column(nullable = false)
    var content: ByteArray,
    @Column(nullable = false)
    var reason: String,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    var user: User
)