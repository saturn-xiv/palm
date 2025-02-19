package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "tag_resources")
@Entity(name = "tag-resource")
class TagResource(
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
    @JoinColumn(name = "tag_id")
    var tag: Tag
)