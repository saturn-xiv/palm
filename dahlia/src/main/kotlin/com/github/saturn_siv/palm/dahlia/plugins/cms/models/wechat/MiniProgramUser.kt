package com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "wechat_mini_program_users")
@Entity(name = "user.by-wechat.mini-program")
class MiniProgramUser(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var unionId: String,
    @Column(nullable = false)
    var appId: String,
    @Column(nullable = false)
    var openId: String,
    @Column
    var nickname: String?,
    @Column
    var avatarUrl: String?,
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
)