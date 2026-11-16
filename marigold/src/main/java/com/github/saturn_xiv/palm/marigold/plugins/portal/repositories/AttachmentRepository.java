package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.util.Optional;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Attachment;

@Repository("portal.r.attachment")
public interface AttachmentRepository extends ListCrudRepository<Attachment, Integer> {
    Optional<Attachment> findBySn(String sn);
}
