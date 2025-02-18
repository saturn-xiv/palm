package com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal

import jakarta.persistence.*
import java.time.Instant

@Table(name = "postal_recipients")
@Entity
class Recipient {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column
    var phone: String? = null

    @Column
    var fax: String? = null

    @Column
    var email: String? = null

    @Column
    var whatsapp: String? = null

    @Column
    var wechat: String? = null

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null
}