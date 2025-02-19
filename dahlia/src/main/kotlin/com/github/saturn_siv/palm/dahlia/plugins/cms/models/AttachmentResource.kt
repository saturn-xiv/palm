package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant


@Table(name = "attachment_resources")
@Entity(name = "attachment.resource")
class AttachmentResource(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var resourceType: String,
    @Column
    var resourceId: Int?,
    @Column(nullable = false)
    var sortOrder: Int,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "attachment_id")
    var attachment: Attachment,
)