package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.CategoryResource
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.cms.r.category-resource.jpa")
interface CategoryResourceRepository : CrudRepository<CategoryResource, Int> {
}