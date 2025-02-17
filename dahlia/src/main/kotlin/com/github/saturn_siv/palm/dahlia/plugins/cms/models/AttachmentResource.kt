package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant


@Table(name = "attachment_resources")
@Entity
class AttachmentResource {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var resourceType: String? = null

    @Column
    var resourceId: Int? = null

    @Column(nullable = false)
    var sortOrder: Int? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "attachment_id")
    val attachment: Attachment? = null
}