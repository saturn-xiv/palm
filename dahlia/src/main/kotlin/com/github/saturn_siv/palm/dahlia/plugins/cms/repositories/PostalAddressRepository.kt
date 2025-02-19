package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.postal.Address
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.cms.r.postal.address.jpa")
interface PostalAddressRepository : CrudRepository<Address, Int> {
}