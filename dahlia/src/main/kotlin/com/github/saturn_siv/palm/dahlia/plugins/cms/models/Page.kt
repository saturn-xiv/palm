package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "pages")
@Entity(name = "page")
class Page(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var lang: String,
    @Column(nullable = false)
    var slug: String,
    @Column(nullable = false)
    var title: String,
    @Column(nullable = false)
    var summary: String,
    @Column(nullable = false)
    var body: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var bodyEditor: Editor,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var template: Template,
    @Column(nullable = false)
    var profile: ByteArray,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status,
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
    enum class Status {
        PENDING, PUBLISHED, LOCKED
    }

    enum class Template {
        BLOG, ARTICLE
    }
}