package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat.MiniProgramUser
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.wechat.Oauth2User
import jakarta.persistence.*
import java.time.Instant

@Table(name = "users")
@Entity(name = "user")
class User(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var uid: String,
    @Column(nullable = false)
    var lang: String,
    @Column(nullable = false)
    var timezone: String,
    @Column(nullable = false)
    var signInCount: Int,
    @Column
    var currentSignInAt: Instant?,
    @Column
    var currentSignInIp: String?,
    @Column
    var lastSignInAt: Instant?,
    @Column
    var lastSignInIp: String?,
    @Column
    var lockedAt: Instant?,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
) {
    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var wechatMiniProgramUsers = mutableSetOf<MiniProgramUser>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var wechatOauth2Users = mutableSetOf<Oauth2User>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var googleOauth2Users = mutableSetOf<GoogleOauth2User>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var emailUsers = mutableSetOf<EmailUser>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var logs = mutableSetOf<Log>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var sessions = mutableSetOf<Session>()

    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "user")
    var attachments = mutableSetOf<Attachment>()
}