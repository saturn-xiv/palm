package com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "wechat.oauth2_users")
@Entity(name = "user.by-wechat.oauth2")
class Oauth2User(
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
    @Column(nullable = false)
    var nickname: String,
    @Column(nullable = false)
    var sex: Int,
    @Column(nullable = false)
    var city: String,
    @Column(nullable = false)
    var province: String,
    @Column(nullable = false)
    var country: String,
    @Column
    var headImgUrl: String?,
    @Column(nullable = false)
    var privilege: ByteArray,
    @Column(nullable = false)
    var lang: String,
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