package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "attachments")
@Entity
class Attachment {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var bucket: String? = null

    @Column(name = "object", nullable = false)
    var object_: String? = null

    @Column(nullable = false)
    var title: String? = null

    @Column(nullable = false)
    var size: Int? = null

    @Column(nullable = false)
    var contentType: String? = null

    @Column
    var uploadedAt: Instant? = null

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "attachment")
    val resources = mutableListOf<AttachmentResource>()

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    val user: User? = null
}