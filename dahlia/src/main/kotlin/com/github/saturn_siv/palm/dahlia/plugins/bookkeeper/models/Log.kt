package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_logs")
@Entity(name = "bookkeeper.log")
class Log(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var operator: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var type: Type,
    @Column(nullable = false)
    var ip: String,
    @Column(nullable = false)
    var message: String,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "team_id")
    var team: Team,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "book_id")
    var book: Book,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "user_id")
    val user: User,
) {
    enum class Type {

    }
}