package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Address
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Recipient
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_vendors")
@Entity(name = "bookkeeper.vendor")
class Vendor(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var name: String,
    @Column(nullable = false)
    var memo: String,
    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor,
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
    @JoinColumn(name = "team_id")
    var team: Team,
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "postal_address_id")
    var address: Address?,
    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "postal_recipient_id")
    var recipient: Recipient?,
) {
    enum class Status {}
}