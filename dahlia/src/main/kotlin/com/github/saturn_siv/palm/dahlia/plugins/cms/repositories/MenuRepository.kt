package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.Menu
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.menu.jpa")
interface MenuRepository : CrudRepository<Menu, Int> {
}