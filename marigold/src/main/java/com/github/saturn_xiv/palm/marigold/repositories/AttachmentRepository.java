package com.github.saturn_xiv.palm.marigold.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.models.Attachment;

@Repository("r.attachment")
public interface AttachmentRepository extends ListCrudRepository<Attachment, Integer> {

}
