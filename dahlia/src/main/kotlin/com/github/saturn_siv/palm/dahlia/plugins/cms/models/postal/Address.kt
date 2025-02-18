package com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal

import jakarta.persistence.*
import java.time.Instant

@Table(name = "postal_addresses")
@Entity
class Address {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int? = null

    @Column(nullable = false)
    var unit: String? = null;

    @Column
    var building: String? = null;

    @Column(nullable = false)
    var street: String? = null;

    @Column(nullable = false)
    var city: String? = null;

    @Column(nullable = false)
    var province: String? = null;

    @Column(nullable = false)
    var country: String? = null;

    @Column(nullable = false)
    var zipCode: String? = null;

    @Column
    var passcode: String? = null;

    @Column
    var googleMap: String? = null;

    @Column
    var aMap: String? = null;

    @Column
    var deletedAt: Instant? = null

    @Column(nullable = false)
    var version: Int? = null

    @Column(nullable = false)
    var updatedAt: Instant? = null

    @Column(nullable = false)
    var createdAt: Instant? = null
}