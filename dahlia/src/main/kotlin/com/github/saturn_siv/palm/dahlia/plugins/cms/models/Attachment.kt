package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "attachments")
@Entity(name = "attachment")
class Attachment(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var bucket: String,
    @Column(name = "object", nullable = false)
    var object_: String,
    @Column(nullable = false)
    var title: String,
    @Column(nullable = false)
    var size: Int,
    @Column(nullable = false)
    var contentType: String,
    @Column
    var uploadedAt: Instant?,
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
) {
    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "attachment")
    var resources = mutableSetOf<AttachmentResource>()
}