package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "logs")
@Entity(name = "log")
class Log(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var plugin: String,
    @Column(nullable = false)
    var ip: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var level: Level,
    @Column(nullable = false)
    var resourceType: String,
    @Column
    var resourceId: Int,
    @Column(nullable = false)
    var message: String,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    val user: User,
) {
    enum class Level {
        DEBUG, INFO, WARN, ERROR
    }
}