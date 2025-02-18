package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_logs")
@Entity
class Log {
    enum class Type {

    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var operator: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var type: Type? = null

    @Column(nullable = false)
    var ip: String? = null

    @Column(nullable = false)
    var message: String? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "team_id")
    val team: Team? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "book_id")
    val book: Book? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    val user: User? = null
}