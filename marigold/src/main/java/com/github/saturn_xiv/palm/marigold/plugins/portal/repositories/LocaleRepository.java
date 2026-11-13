package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Locale;

@Repository("portal.r.locale")
public interface LocaleRepository extends ListCrudRepository<Locale, Integer> {

}
