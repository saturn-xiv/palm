package com.github.saturn_siv.palm.dahlia.plugins.bbs.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bbs_topics")
@Entity(name = "bbs.topic")
class Topic(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var slug: String,
    @Column(nullable = false)
    var title: String,
    @Column(nullable = false)
    var body: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var bodyEditor: Editor,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status,
    @Column(nullable = false)
    var profile: ByteArray,
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
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "forum_id")
    var forum: Forum,
) {
    enum class Status {
        OPENING, LOCKED
    }
}