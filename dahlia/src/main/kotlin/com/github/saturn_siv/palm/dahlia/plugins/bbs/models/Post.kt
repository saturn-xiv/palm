package com.github.saturn_siv.palm.dahlia.plugins.bbs.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bbs_posts")
@Entity(name = "bbs.post")
class Post(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
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
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "topic_id")
    var topic: Topic,
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "parent_id")
    var parent: Post?
) {
    enum class Status {
        OPENING, LOCKED
    }
}