package com.github.saturn_siv.palm.dahlia.plugins.cms.repositories

import com.github.saturn_siv.palm.dahlia.plugins.cms.models.AttachmentResource
import org.springframework.data.repository.CrudRepository
import org.springframework.stereotype.Repository

@Repository("dahlia.cms.r.attachment-resource.jpa")
interface AttachmentResourceRepository : CrudRepository<AttachmentResource, Int> {
}