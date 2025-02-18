package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "currencies")
@Entity
class Currency {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var code: String? = null

    @Column(nullable = false)
    var number: String? = null

    @Column(nullable = false)
    var name: String? = null

    @Column(nullable = false)
    var country: String? = null

    @Column(nullable = false)
    var units: Int? = null

    @Column(nullable = false)
    var createdAt: Instant? = null
}