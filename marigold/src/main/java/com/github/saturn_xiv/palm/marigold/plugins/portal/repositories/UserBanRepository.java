package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Ban;

@Repository("portal.r.user-ban")
public interface UserBanRepository extends ListCrudRepository<Ban, Integer> {

}
