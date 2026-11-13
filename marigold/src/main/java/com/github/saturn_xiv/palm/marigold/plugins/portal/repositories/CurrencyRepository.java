package com.github.saturn_xiv.palm.marigold.plugins.portal.repositories;

import org.springframework.data.repository.ListCrudRepository;
import org.springframework.stereotype.Repository;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Currency;

@Repository("portal.r.currency")
public interface CurrencyRepository extends ListCrudRepository<Currency, Integer> {

}
