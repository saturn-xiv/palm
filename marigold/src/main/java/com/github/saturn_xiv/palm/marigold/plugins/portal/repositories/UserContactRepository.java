package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Contact;

@Repository("portal.r.contact")
public interface UserContactRepository extends ListCrudRepository<Contact, Integer> {

}
