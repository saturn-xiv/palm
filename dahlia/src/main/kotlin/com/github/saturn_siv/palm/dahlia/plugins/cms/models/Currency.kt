package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "currencies")
@Entity(name = "currency")
class Currency(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var code: String,
    @Column(nullable = false)
    var number: String,
    @Column(nullable = false)
    var name: String,
    @Column(nullable = false)
    var country: String,
    @Column(nullable = false)
    var units: Int,
    @Column(nullable = false)
    var createdAt: Instant,
)