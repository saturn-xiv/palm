package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.ByEmail;

@Repository("portal.r.user.by-email")
public interface EmailUserRepository extends ListCrudRepository<ByEmail, Integer> {

}
