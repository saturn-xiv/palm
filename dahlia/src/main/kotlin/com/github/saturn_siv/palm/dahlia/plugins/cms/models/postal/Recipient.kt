package com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal

import jakarta.persistence.*
import java.time.Instant

@Table(name = "postal_recipients")
@Entity(name = "postal.recipient")
class Recipient(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column
    var phone: String?,
    @Column
    var fax: String?,
    @Column
    var email: String?,
    @Column
    var whatsapp: String?,
    @Column
    var wechat: String?,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
)