package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Category;

@Repository("portal.r.category")
public interface CategoryRepository extends ListCrudRepository<Category, Integer> {

}
