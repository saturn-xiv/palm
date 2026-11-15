package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import java.util.Optional;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;

@Repository("portal.r.user")
public interface UserRepository extends ListCrudRepository<Item, Integer> {
    Optional<Item> findBySn(String sn);
}
