package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.AttachmentResource
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("jpa.dahlia.cms.r.attachment-resource")
interface AttachmentResourceRepository : CrudRepository<AttachmentResource, Int>,
    JpaRepository<AttachmentResource, Int> {
}