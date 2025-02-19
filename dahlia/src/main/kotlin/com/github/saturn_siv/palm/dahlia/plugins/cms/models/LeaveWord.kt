package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "leave_words")
@Entity(name = "leave-word")
class LeaveWord(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var lang: String,
    @Column(nullable = false)
    var ip: String,
    @Column(nullable = false)
    var resourceType: String,
    @Column
    var resourceId: Int?,
    @Column(nullable = false)
    var body: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var bodyEditor: Editor,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
) {
    enum class Status {
        PENDING, PUBLISHED
    }
}