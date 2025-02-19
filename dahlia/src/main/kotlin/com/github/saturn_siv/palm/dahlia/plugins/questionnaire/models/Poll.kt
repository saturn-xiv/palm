package com.github.saturn_siv.palm.dahlia.plugins.questionnaire.models

import jakarta.persistence.*
import java.time.Instant

@Table(name = "questionnaire_polls")
@Entity(name = "questionnaire.poll")
class Poll(
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(nullable = false)
    var id: Int,
    @Column(nullable = false)
    var batchNo: String,
    @Column(nullable = false)
    var value: ByteArray,
    @Column(nullable = false)
    var ip: String,
    @Column(nullable = false)
    var version: Int,
    @Column(nullable = false)
    var updatedAt: Instant,
    @Column(nullable = false)
    var createdAt: Instant,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "form_id")
    val form: Form,
    @ManyToOne(fetch = FetchType.LAZY, optional = false)
    @JoinColumn(name = "field_id")
    val field: Field,
)