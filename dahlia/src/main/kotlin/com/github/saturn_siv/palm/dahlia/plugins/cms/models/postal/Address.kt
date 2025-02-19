package com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal

import jakarta.persistence.*
import java.time.Instant

@Table(name = "postal_addresses")
@Entity(name = "postal.address")
class Address(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var unit: String,
    @Column
    var building: String?,
    @Column(nullable = false)
    var street: String,
    @Column(nullable = false)
    var city: String,
    @Column(nullable = false)
    var province: String,
    @Column(nullable = false)
    var country: String,
    @Column(nullable = false)
    var zipCode: String,
    @Column
    var passcode: String?,
    @Column
    var googleMap: String?,
    @Column
    var aMap: String?,
    @Column
    var deletedAt: Instant?,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
)