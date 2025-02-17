package com.github.saturn_siv.palm.dahlia.plugins.bbs.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bbs_posts")
@Entity
class Post {
    enum class Status {
        OPENING, LOCKED
    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var body: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var bodyEditor: Editor? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status? = null

    @Column(nullable = false)
    var profile: ByteArray? = null

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    val user: User? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "forum_id")
    val forum: Forum? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "topic_id")
    val topic: Topic? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "parent_id")
    val parent: Post? = null
}