package com.github.saturn_siv.palm.dahlia.plugins.bookkeeper.models

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Editor
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Address
import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Recipient
import jakarta.persistence.*
import java.time.Instant

@Table(name = "bookkeeper_vendors")
@Entity
class Vendor {
    enum class Status {}

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var name: String? = null

    @Column(nullable = false)
    var memo: String? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var memoEditor: Editor? = null

    @Column(nullable = false)
    var profile: ByteArray? = null

    @Enumerated(EnumType.STRING)
    @Column(nullable = false)
    var status: Status? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "team_id")
    val team: Team? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "postal_address_id")
    val address: Address? = null

    @ManyToOne(fetch = FetchType.LAZY, optional = true)
    @JoinColumn(name = "postal_recipient_id")
    val recipient: Recipient? = null
}