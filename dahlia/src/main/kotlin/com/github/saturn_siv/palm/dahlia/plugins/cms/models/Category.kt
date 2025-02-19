package com.github.saturn_siv.palm.dahlia.plugins.cms.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "categories")
@Entity(name = "category")
class Category(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var lang: String,
    @Column(nullable = false)
    var location: String,
    @Column(nullable = false)
    var label: String,
    @Column(nullable = false)
    var left: Int,
    @Column(nullable = false)
    var right: Int,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,

    ) {
    @OneToMany(cascade = [(CascadeType.ALL)], fetch = FetchType.LAZY, mappedBy = "category")
    var resources = mutableSetOf<CategoryResource>()
}