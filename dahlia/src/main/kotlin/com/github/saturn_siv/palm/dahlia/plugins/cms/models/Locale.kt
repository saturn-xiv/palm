package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "locales")
@Entity(name = "locale")
class Locale(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var lang: String,
    @Column(nullable = false)
    var code: String,
    @Column(nullable = false)
    var message: String,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
)