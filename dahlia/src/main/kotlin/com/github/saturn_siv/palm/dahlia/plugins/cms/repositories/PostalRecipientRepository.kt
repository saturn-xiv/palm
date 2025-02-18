package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Recipient
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.cms.r.postal.recipient")
interface PostalRecipientRepository : CrudRepository<Recipient, Int>, JpaRepository<Recipient, Int> {
}